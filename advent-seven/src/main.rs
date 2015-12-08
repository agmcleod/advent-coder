extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;
use regex::Regex;

type ArgumentMap = HashMap<Operation, Regex>;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operation {
    Lshift,
    Rshift,
    And,
    Or,
    Not,
    Set,
    Wire,
    Value
}

fn insert_operation(map: &mut ArgumentMap, operation: Operation, value: &str) {
    map.insert(operation, Regex::new(value).unwrap());
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("commands.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let mut argument_map: ArgumentMap = ArgumentMap::new();
    insert_operation(&mut argument_map, Operation::Lshift, r"LSHIFT");
    insert_operation(&mut argument_map, Operation::Rshift, r"RSHIFT");
    insert_operation(&mut argument_map, Operation::And, r"AND");
    insert_operation(&mut argument_map, Operation::Or, r"OR");
    insert_operation(&mut argument_map, Operation::Not, r"NOT");
    insert_operation(&mut argument_map, Operation::Set, r"->");
    insert_operation(&mut argument_map, Operation::Wire, r"[a-z]+");
    insert_operation(&mut argument_map, Operation::Value, r"\d+");

    let wires: HashMap<&str, usize> = HashMap::new();
    match read_text() {
        Ok(text) => {
            let lines: Vec<&str> = text.split("\n").collect();
            for line in lines.iter() {
                if *line == "" {
                    continue;
                } else {
                    let words: Vec<&str> = line.split(" ").collect();
                    let mut wire: usize = 0;
                    let mut wire_key: String = String::new();
                    let mut amount: usize = 0;
                    for word in words.iter() {
                        if argument_map.get(&Operation::Lshift).unwrap().is_match(word) {
                            wire << amount;
                        } else if argument_map.get(&Operation::Rshift).unwrap().is_match(word) {
                            wire >> amount;
                        } else if argument_map.get(&Operation::And).unwrap().is_match(word) {

                        } else if argument_map.get(&Operation::Not).unwrap().is_match(word) {

                        } else if argument_map.get(&Operation::Set).unwrap().is_match(word) {
                            
                        } else if argument_map.get(&Operation::Wire).unwrap().is_match(word) {
                            wire_key = String::from(*word);
                        } else if argument_map.get(&Operation::Value).unwrap().is_match(word) {
                            amount = word.parse().ok().unwrap();
                        }
                    }
                }
            }
        },
        Err(err) => panic!("Could not read file {:?}", err)
    }
}
