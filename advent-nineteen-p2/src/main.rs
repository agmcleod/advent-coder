extern crate regex;
extern crate rand;

use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use rand::distributions::{IndependentSample, Range};

fn get_potential_replacements(transform_map: &HashMap<String, String>, value: &str) -> Vec<String> {
    let mut replacements = Vec::<String>::new();
    for (key, _) in transform_map.iter() {
        if value.contains(key) {
            replacements.push(key.clone());
        }
    }

    replacements
}

fn replace_permutation(string_to_replace: &str, value: &str, start_i: &usize, end_i: &usize) -> String {
    let bytes = string_to_replace.as_bytes();
    let mut bytes = bytes.iter().collect::<Vec<&u8>>();
    for _ in (*start_i..*end_i) {
        bytes.remove(*start_i);
    }
    let value_bytes = value.as_bytes();
    let mut i = 0;
    for b in value_bytes.iter() {
        bytes.insert(start_i + i, b);
        i += 1;
    }

    String::from_utf8(bytes.iter().map(|&b| *b).collect()).unwrap()
}

fn try_replacements(transform_map: &HashMap<String, String>, value: &str, permutations: &mut Vec<String>, moleocule: &str) -> usize {
    let replacements = get_potential_replacements(transform_map, value);
    if replacements.len() == 0 {
        try_replacements(transform_map, moleocule, permutations, moleocule)
    } else {
        let mut rng = rand::thread_rng();

        let mut rand_index = 0;
        if replacements.len() > 1 {
            rand_index = Range::new(0, replacements.len() - 1).ind_sample(&mut rng);
        }
        let replacement = replacements.get(rand_index).unwrap();
        let re = Regex::new(replacement).unwrap();
        let (start_i, end_i) = re.find(value).unwrap();
        let modified_value = replace_permutation(&value, transform_map.get(replacement).unwrap(), &start_i, &end_i);
        if modified_value != "e" && !permutations.contains(&modified_value) {
            let copied_value = modified_value.clone();
            permutations.push(copied_value);
            return 1 + try_replacements(transform_map, &modified_value, permutations, moleocule)
        } else if modified_value == "e" {
            return 1
        } else {
            try_replacements(transform_map, &modified_value, permutations, moleocule)
        }
    }
}

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
    let mut transform_map: HashMap<String, String> = HashMap::new();
    let mut permutations = Vec::<String>::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }

        if line.contains("=>") {
            let parts = line.split(" => ").collect::<Vec<&str>>();
            transform_map.insert(String::from(parts[1]), String::from(parts[0]));
        } else {
            molecule = line.clone();
        }
    }

    println!("{:?}", try_replacements(&transform_map, &molecule, &mut permutations, molecule));
}
