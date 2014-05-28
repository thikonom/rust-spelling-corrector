#![feature(phase)]
extern crate regex;
#[phase(syntax)] extern crate regex_macros;

extern crate collections;
use std::io::File;
use std::io::BufferedReader;
use collections::HashMap;
use collections::HashSet;
use std::str;
use std::string::String;


fn train<'a>(contents: &'a str) -> HashMap<&'a str, uint> {
  // Returns a HashMap
  // with (key, value) = (`word`, `#times the word occurs in the doc`)
  let mut nwords = HashMap::<&str, uint>::new();
  let re = regex!(r"([A-Za-z]+)");

  let words: Vec<&str> = re.captures_iter(contents)
                         .map(|x| x.at(0))
                         .collect();

  for word in words.iter() {
    nwords.insert_or_update_with(
      *word, 1,
      |_key, already| { *already += 1 }
    );
  }

  nwords
}

fn edits1(word: &str) -> HashSet<String>{
  // Returns a set of all the edits of the word
  // with edit distance = 1
  let w_len = word.len();
  let alphabet = "abcdefghijklmnopqrstuvwxyz";
  let mut splits: Vec<(&str, &str)> = vec![];

  for i in range(0, w_len + 1) {
    splits.push((word.slice(0, i), word.slice(i, w_len)))
  }

  let deletes = splits.iter().filter_map(|&(x,y)|(
       if y.len() > 0 {
         Some(x.to_owned().append(y.slice_from(1)))
       }
       else { None }
  )).collect::<Vec<String>>();

  let transposes = splits.iter().filter_map(|&(x,y)|(
       if y.len() > 1 {
         Some([x.to_owned(),
               str::from_utf8([y[1], y[0]]).unwrap().to_owned(),
               y.slice_from(2).to_owned()].concat())
       }
       else { None }
  )).collect::<Vec<String>>();

  let mut replaces: Vec<String> = vec![];
  let mut inserts: Vec<String> = vec![];

  for &(x, y) in splits.iter() {
    for ch in alphabet.chars() {
      if y.len() > 0 {
        replaces.push([x.to_owned(),
                       ch.to_str(),
                       y.slice_from(1).to_owned()]
                .concat());
      }
      inserts.push([x.to_owned(),
                    ch.to_str(),
                    y.to_owned()]
                .concat());
    }
  }

  let mut edits = HashSet::<String>::new();
  for x in deletes.move_iter()
            .chain(transposes.move_iter())
            .chain(replaces.move_iter())
            .chain(inserts.move_iter()) {
    edits.insert(x.to_str());
  }

  edits
}

fn known_edits2<'a>(word: &'a str, candidates: &HashMap<&'a str, uint>) -> HashSet<String> {
  // Returns a set of all the edits of the word
  // with edit distance = 2
  let mut edits = HashSet::<String>::new();
  for e1 in edits1(word).iter() {
    for e2 in edits1(e1.as_slice()).iter() {
      if candidates.contains_key_equiv(&e2.as_slice()) {
        edits.insert(e2.to_str());
      }
    }
  }

  edits
}

fn known<'a>(words: Vec<String>, candidates: &HashMap<&'a str, uint>) -> HashSet<String> {
  // Returns the words we have seen in the language model training data

  let mut known_words = HashSet::<String>::new();
  for word in words.iter() {
    if candidates.contains_key_equiv(word) {
      known_words.insert(word.to_str());
    }
  }

  known_words
}

fn correct<'a>(word: &'a String, candidates: &HashMap<&'a str, uint>) -> (String, uint) {
   // Returns the possible correct words
   // from the mispelled ones
   let mut cs: HashSet<String> = HashSet::<String>::new();
   let x = known(vec![word.to_owned()], candidates);
   if x.is_empty() {
     let k = edits1(word.as_slice())
              .move_iter()
              .collect::<Vec<String>>();
     let y = known(k, candidates);
     if y.is_empty() {
        let z = known_edits2((*word).as_slice(), candidates);
        if z.is_empty() {
          cs.insert(word.to_owned());
        }
        else { cs = z; }
     }
     else { cs = y; }
   }
   else { cs = x; }

   let mut max_freq = 0u;
   let mut max_word = String::new();
   for w in cs.move_iter() {
     match candidates.find_equiv(&(w.as_slice())) {
       Some(freq)  => {
        if *freq > max_freq {
          max_word = w;
          max_freq = *freq;
        }
       },
       None => {
        if max_freq==0u {
          max_word = w;
          max_freq = max_freq;
        }
       }
     }
   }
   (max_word, max_freq)
  }


#[test]
fn returns_correct_words() {
  let path = Path::new("big.txt");
  let mut file = BufferedReader::new(File::open(&path));
  let contents = file.read_to_str().unwrap();
  let nwords = train(contents.as_slice());

  let targets = vec!["access".to_strbuf(), "accommodation".to_strbuf(), "supposedly".to_strbuf(), "decisions".to_strbuf()];
  let wrongs  = vec!["acess".to_strbuf(), "acomodation".to_strbuf(), "supposidly".to_strbuf(), "deciscions".to_strbuf()];

  for (target, wrong) in targets.iter().zip(wrongs.iter()) {
    let (w, _) = correct(wrong, &nwords);
    let t = target.to_strbuf();
    assert!(t==w);
  }
}
