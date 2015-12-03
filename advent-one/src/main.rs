use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn read_text() -> Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open("in.txt"));
  try!(file.read_to_string(&mut text));
  Ok(text)
}

fn main() {
  match read_text() {
    Ok(text) => {
      let v1: Vec<&str> = text.split("(").collect();
      let v2: Vec<&str> = text.split(")").collect();
      let went_up = v1.len() - 1;
      let went_down = v2.len() - 1;
      let floor = went_up - went_down;
      println!("On Floor: {:?}", floor);
    },
    Err(reason) => panic!("{:?}", reason),
  }
  
}
