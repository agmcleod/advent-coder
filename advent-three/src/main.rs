use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

type Coords = HashMap<i32, Vec<i32>>;

fn read_text() -> Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open("in.txt"));
  try!(file.read_to_string(&mut text));
  Ok(text)
}

fn run_instruction(map: &mut Coords, x: i32, y: i32) -> usize {
  if map.contains_key(&x) {
    let mut vec = &mut map.get_mut(&x).unwrap();
    if !vec.contains(&y) {
      vec.push(y);
      1
    } else {
      0
    }
  } else {
    let vec: Vec<i32> = vec![y];
    map.insert(x, vec);
    1
  }
}

fn main() {
  match read_text() {
    Ok(text) => {
      let mut x: i32 = 0;
      let mut y: i32 = 0;

      let mut map: Coords = HashMap::new();
      map.insert(x, vec![y]);

      let instructions: Vec<&str> = text.split("").collect();
      let mut house_count = 1;
      for instruction in instructions.iter() {
        match *instruction {
          ">" => {
            x += 1
          },
          "<" => {
            x -= 1
          },
          "^" => {
            y -= 1
          },
          "v" => {
            y += 1
          },
          _ => continue
        }

        house_count += run_instruction(&mut map, x, y);
      }

      println!("houses given gifts {:?}", house_count);
    },
    Err(reason) => panic!("{:?}", reason),
  }
}
