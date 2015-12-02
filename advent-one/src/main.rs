use std::fs::File;
use std::io::prelude::*;

fn read_text(text: &mut String) -> std::io::Result<()> {
  let mut file = try!(File::open("in.txt"));
  try!(file.read_to_string(text));
  Ok(())
}

fn main() {
  let mut text = String::new();
  read_text(&mut text);
  let v1: Vec<&str> = text.split("(").collect();
  let v2: Vec<&str> = text.split(")").collect();
  let went_up = v1.len() - 1;
  let went_down = v2.len() - 1;
  let floor = went_up - went_down;
  println!("On Floor: {:?}", floor);
}
