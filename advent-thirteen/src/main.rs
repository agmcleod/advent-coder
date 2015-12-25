extern crate permutohedron;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;
use permutohedron::Heap;

fn parse_line<'a>(people: &mut Vec<&'a str>, relationships: &mut HashMap<&'a str, Vec<(&'a str, i16)>>, line: &'a str) {
    let bits = line.split(" ").collect::<Vec<&str>>();
    if !people.contains(&bits[0]) {
        people.push(bits[0]);
    }

    if !relationships.contains_key(&bits[0]) {
        let mut relations: Vec<(&'a str, i16)> = Vec::new();
        relationships.insert(bits[0], relations);
    }

    let mut relations = relationships.get_mut(&bits[0]).unwrap();
    let mut num: i16 = bits[3].parse().ok().expect("nope");
    if bits[2] == "lose" {
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

    let mut people: Vec<&str> = Vec::new();
    let mut relationships: HashMap<&str, Vec<(&str, i16)>> = HashMap::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }
        parse_line(&mut people, &mut relationships, line);
    }

    let len = people.len();
    let permutations = Heap::new(&mut people[..len]).collect::<Vec<_>>();
    let mut happiness_values: Vec<i16> = Vec::new();
    for set in permutations.iter() {
        let mut happiness = 0i16;

        let mut i = 0;
        let len = set.len();
        for person in set.iter() {
            let neighbours = relationships.get(person).unwrap();
            let mut left_index = i - 1i16;
            if left_index < 0 {
                left_index = len as i16 - 1i16;
            }
            let left = set[left_index as usize];

            let mut right_index = i + 1i16;
            if right_index == len as i16 {
                right_index = 0;
            }

            let right = set[right_index as usize];

            for neighbour in neighbours.iter() {
                let (name, value) = *neighbour;
                if name == left || name == right {
                    happiness += value;
                }
            }

            i += 1;
        }

        happiness_values.push(happiness);
    }
    happiness_values.sort();
    println!("{:?}", happiness_values[happiness_values.len() - 1]);
}
