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
      let lines: Vec<&str> = text.split("\n").collect();
      let mut total: u32 = 0;
      let mut line_count = 1;
      let mut feet_of_ribbon = 0;
      for line in lines.iter() {
        let parts: Vec<&str> = line.split("x").collect();

        // only continue first, as that should catch a bad line.
        let l: u32 = match parts[0].trim().parse() {
          Ok(num) => num,
          Err(_) => {
            println!("Line {:?} could not be parsed, its contents are: {:?}", line_count, line);
            continue
          }
        };
        let w: u32 = parts[1].trim().parse().ok().expect("H was not a number");
        let h: u32 = parts[2].trim().parse().ok().expect("W was not a number");

        let bottom: u32 = l*w;
        let side: u32 = w*h;
        let front: u32 = h*l;

        total += 2*bottom + 2*side + 2*front;
        let mut sides: Vec<u32> = vec![bottom, side, front];
        sides.sort();
        total += sides[0];

        let mut dimensions: Vec<u32> = vec![l, w, h];
        dimensions.sort();

        feet_of_ribbon += dimensions[0] * 2 + dimensions[1] * 2 + l*w*h;
        
        line_count += 1;
      }

      println!("Total: {:?}", total);
      println!("Amount of ribbon: {:?}", feet_of_ribbon);
    },
    Err(error) => panic!("{:?}", error)
  }
}
