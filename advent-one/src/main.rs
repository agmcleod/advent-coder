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
      let characters: Vec<&str> = text.split("").collect();
      let mut floor = 0;
      let mut position = 0;
      for character in characters.iter() {
        if *character == "(" {
          floor += 1;
        } else if *character == ")" {
          floor -= 1;
        }
        if floor == -1 {
          break;
        }
        position += 1;
      }
      println!("Position: {:?}", position);
    },
    Err(reason) => panic!("{:?}", reason),
  }
  
}
