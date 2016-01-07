extern crate regex;

use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn read_file() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("input.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_file() {
        Ok(text) => text,
        Err(e) => panic!("{:?}", e)
    };

    let mut molecule = "";
    let mut transform_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut permutations: Vec<&str> = Vec::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }

        if line.contains("=>") {
            let parts = line.split(" => ").collect::<Vec<&str>>();
            if transform_map.contains_key(parts[0]) {
                transform_map.get_mut(parts[0]).unwrap().push(parts[1]);
            } else {
                transform_map.insert(parts[0], vec![parts[1]]);
            }
        } else {
            molecule = line.clone();
        }
    }

    for (replace_target, values) in transform_map.iter() {
        let re = Regex::new(values[0]).unwrap();
        for c in re.find_iter(molecule) {
            println!("{:?}", c);
        }
    }

    println!("{:?}", permutations.len());
}
