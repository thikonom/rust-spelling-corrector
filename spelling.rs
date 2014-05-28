#![feature(phase)]
extern crate regex;
#[phase(syntax)] extern crate regex_macros;

extern crate collections;
use collections::HashMap;
use collections::HashSet;
use std::io::File;
use std::io::BufferedReader;
use std::str;
use std::string::String;


fn train<'a>(contents: &'a str) -> HashMap<&'a str, uint> {
  let mut nwords = HashMap::<&str, uint>::new();
  let re = regex!(r"([A-Za-z]+)");

  let words: Vec<&str> = re.captures_iter(contents)
                         .map(|x| x.at(0))
                         .collect();

  for word in words.iter() {
    nwords.insert_or_update_with(
      *word, 1,
      |_key, already| { *already += 1}
    );
  }

  nwords
}

fn edits1(word: &str) -> HashSet<String>{
  let w_len = word.len();
  let alphabet = "abcdefghijklmnopqrstuvwxyz";
  let mut splits: Vec<(&str, &str)> = vec![];

  for i in range(0, w_len + 1) {
    splits.push((word.slice(0, i), word.slice(i, w_len)))
  }

  println!("SPLITS");
  for tpl in splits.iter() {
    println!("{}", tpl);
  }

  let deletes = splits.iter().filter_map(|&(x,y)|(
       if y.len() > 0 {
         Some(x.to_owned().append(y.slice_from(1)))
       }
       else { None }
  )).collect::<Vec<String>>();

  println!("DELETES");
  for tpl in deletes.iter() {
    println!("{}", tpl);
  }

  let transposes = splits.iter().filter_map(|&(x,y)|(
       if y.len() > 1 {
         Some([x.to_owned(),
               str::from_utf8([y[1], y[0]]).unwrap().to_owned(),
               y.slice_from(2).to_owned()].concat())
       }
       else { None }
  )).collect::<Vec<String>>();

  println!("TRANSPOSES");
  for tpl in transposes.iter() {
    println!("{}", tpl);
  }

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

  println!("REPLACES");
  for tpl in replaces.iter() {
    println!("{}", tpl);
  }
  println!("INSERTS");
  for tpl in inserts.iter() {
    println!("{}", tpl);
  }

  let mut set = HashSet::<String>::new();
  let all = deletes + transposes + replaces + inserts;
  for x in all.iter() {
    set.insert(x.to_str());
  }

  set
}

fn main() {
  let path = Path::new("small.txt");
  let mut file = BufferedReader::new(File::open(&path));
  let contents = file.read_to_str().unwrap();

  let nwords = train(contents.as_slice());
}
