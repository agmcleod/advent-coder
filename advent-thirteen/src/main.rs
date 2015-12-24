use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn parse_line<'a>(happiness_units: &mut HashMap<&'a str, i16>, relationships: &mut HashMap<&'a str, Vec<(&'a str, i16)>>, line: &'a str) {
    let bits = line.split(" ").collect::<Vec<&str>>();
    if !happiness_units.contains_key(&bits[0]) {
        happiness_units.insert(bits[0], 0);
    }

    if !relationships.contains_key(&bits[0]) {
        let mut relations: Vec<(&'a str, i16)> = Vec::new();
        relationships.insert(bits[0], relations);
    }

    let mut relations = relationships.get_mut(&bits[0]).unwrap();
    let mut num: i16 = bits[3].parse().ok().expect("nope");
    if bits[4] == "lose" {
        num *= -1;
    }
    relations.push((&bits[10], num));
}

fn read_text() -> Result<String> {
  let mut text = String::new();
  let mut file = try!(File::open("rules.txt"));
  try!(file.read_to_string(&mut text));
  Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let text = text.replace(".", "");

    let mut happiness_units: HashMap<&str, i16> = HashMap::new();
    let mut relationships: HashMap<&str, Vec<(&str, i16)>> = HashMap::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }
        parse_line(&mut happiness_units, &mut relationships, line);
    }

    println!("{:?}", relationships);
}
