use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

type Coords = HashMap<i32, Vec<i32>>;

enum InstructionError {
  InvalidInstruction
}

fn modify_values_from_instruction(instruction: &str, x: &mut i32, y: &mut i32) -> std::result::Result<i32, InstructionError> {
  match instruction {
    ">" => {
      *x += 1;
      Ok(1)
    },
    "<" => {
      *x -= 1;
      Ok(1)
    },
    "^" => {
      *y -= 1;
      Ok(1)
    },
    "v" => {
      *y += 1;
      Ok(1)
    },
    _ => Err(InstructionError::InvalidInstruction)
  }
}

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

fn run_part_one(text: String) -> usize {
  let mut x: i32 = 0;
  let mut y: i32 = 0;

  let mut map: Coords = HashMap::new();
  map.insert(x, vec![y]);

  let instructions: Vec<&str> = text.split("").collect();
  let mut house_count = 1;
  for instruction in instructions.iter() {
    match modify_values_from_instruction(instruction, &mut x, &mut y) {
      Ok(i) => {
        house_count += run_instruction(&mut map, x, y);
      },
      Err(err) => continue
    }
  }

  house_count
}

fn run_part_two(text: String) -> usize {
  let mut x: i32 = 0;
  let mut y: i32 = 0;

  let mut rx: i32 = 0;
  let mut ry: i32 = 0;

  let mut map: Coords = HashMap::new();
  map.insert(x, vec![y]);

  let mut house_count = 1;
  let instructions: Vec<&str> = text.split("").collect();
  let mut index = 0;
  for instruction in instructions.iter() {
    let mut modx;
    let mut mody;
    if index % 2 == 0 {
      modx = &mut x;
      mody = &mut y;
    } else {
      modx = &mut rx;
      mody = &mut ry;
    }
    match modify_values_from_instruction(&instruction, modx, mody) {
      Ok(i) => {
        house_count += run_instruction(&mut map, *modx, *mody);
      },
      Err(err) => continue
    }

    index += 1;
  }

  house_count
}

fn main() {
  match read_text() {
    Ok(text) => {
      println!("houses given gifts {:?}", run_part_two(text));
    },
    Err(reason) => panic!("{:?}", reason),
  }
}
