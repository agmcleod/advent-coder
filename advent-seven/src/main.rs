extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;
use regex::Regex;

#[derive(Eq, PartialEq, Hash)]
enum Operation {
    Lshift,
    Rshift,
    And,
    Or,
    Not
}

struct Instruction<'a> {
    left: Option<&'a str>,
    operation: Option<&'a Operation>,
    right: Option< &'a str>,
    target: Option<&'a str>
}

impl<'a> Instruction<'a> {
    fn new(left: Option<&'a str>, operation: Option<&'a Operation>, right: Option<&'a str>, target: Option<&'a str>) -> Instruction<'a> {
        Instruction{ left: left, operation: operation, right: right, target: target }
    }
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("commands.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(err) => panic!("Could not read file {:?}", err)
    };

    let mut operation_map: HashMap<&str, Operation> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut wires: HashMap<&str, i16> = HashMap::new();
    operation_map.insert("LSHIFT", Operation::Lshift);
    operation_map.insert("RSHIFT", Operation::Rshift);
    operation_map.insert("AND", Operation::And);
    operation_map.insert("OR", Operation::Or);
    operation_map.insert("NOT", Operation::Not);
    let lines: Vec<&str> = text.split("\n").collect();
    for line in lines.iter() {
        if *line == "" {
            continue;
        } else {
            let words: Vec<&str> = text.split("\n").collect();
            if words.len() == 5 {
                instructions.push(Instruction::new(
                    Some(words[0]), operation_map.get(words[1]), Some(words[2]), Some(words[4])
                ));
            } else if words.len() == 4 {
                instructions.push(Instruction::new(
                    None, operation_map.get(words[0]), Some(words[1]), Some(words[3])
                ));
            } else if words.len() == 3 {
                instructions.push(Instruction::new(
                    Some(words[0]), None, None, Some(words[2])
                ));
            }

        }
    }
}
