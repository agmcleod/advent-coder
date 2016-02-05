extern crate regex;
extern crate rand;

use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use rand::Rang;

fn get_potential_replacements(transform_map: &HashMap<&str, &str>, value: &str) -> Vec<&str> {
    let mut replacements = Vec::new();
    for (key, _) in transform_map.iter() {
        if replacements.contains(key) {
            replacements.push(key);
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

fn try_replacements(transform_map: &HashMap<&str, &str>, value: &str, permutations: &mut Vec<String>) -> usize {
    let replacements = get_potential_replacements(transform_map, value);
    let rand_index =
    for replacement in replacements.iter() {
        if value.contains(key) {
            let re = Regex::new(key).unwrap();
            for (start_i, end_i) in re.find_iter(value) {
                let modified_value = replace_permutation(&value, &replacement, &start_i, &end_i);
                *count += 1;
                if modified_value != target_value &&
                !permutations.contains(&modified_value) &&
                modified_value.len() < target_value.len() {
                    let copied_value = modified_value.clone();
                    permutations.push(copied_value);
                    for (key, values) in transform_map.iter() {
                        if modified_value.contains(key) {
                            let mut cloned_count = count.clone();
                            return 1 + try_replacements(
                                transform_map, &modified_value, permutations
                            );
                        }
                    }
                } else if modified_value == target_value {
                    steps.push(*count);
                }
            }
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
    let mut transform_map: HashMap<&str, &str> = HashMap::new();
    let mut permutations = Vec::<String>::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }

        if line.contains("=>") {
            let parts = line.split(" => ").collect::<Vec<&str>>();
            transform_map.insert(parts[1], parts[0]);
        } else {
            molecule = line.clone();
        }
    }

    println!("{:?}", try_replacements(&transform_map, &molecule, &replacements, &mut permutations));
}
