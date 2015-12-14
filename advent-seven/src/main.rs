extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operation {
    Lshift,
    Rshift,
    And,
    Or,
    Not
}

#[derive(Debug)]
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

fn parse_or_get_from_map(num_regex: &Regex, value: &str, wires: &HashMap<&str, u16>) -> Option<u16> {
    if num_regex.is_match(value) {
        Some(value.parse().ok().expect("Could not parse"))
    } else if wires.contains_key(&value) {
        Some(*wires.get(&value).unwrap())
    } else {
        None
    }
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("commands.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn run_operation(left: &u16, right: &u16, operation: &Operation) -> u16 {
    match operation {
        &Operation::Lshift => left << right,
        &Operation::Rshift => left >> right,
        &Operation::And => left & right,
        &Operation::Or => left | right,
        &Operation::Not => !right
    }
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(err) => panic!("Could not read file {:?}", err)
    };

    let mut operation_map: HashMap<&str, Operation> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

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
            let words: Vec<&str> = line.split(" ").collect();
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

    let mut wires: HashMap<&str, u16> = HashMap::new();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut processed_lines: Vec<usize> = Vec::new();
    let mut i = 0;
    loop {
        if i >= instructions.len() {
            i = 0;
        }
        if processed_lines.contains(&i) {
            i += 1;
            continue
        }
        let instruction = instructions.get(i).unwrap();
        match instruction.left {
            Some(wire) => {
                let left: u16 = match parse_or_get_from_map(&num_regex, wire, &wires) {
                    Some(n) => n,
                    None => {
                        i += 1;
                        continue
                    }
                };
                let mut write_value = 0;
                let mut wire_to_write: &str = "";
                {
                    match instruction.operation {
                        Some(operation) => {
                            if instruction.right == None {
                                panic!("No right {:?}", instruction);
                            }
                            let right = instruction.right.unwrap();
                            let num: u16 = match parse_or_get_from_map(&num_regex, right, &wires) {
                                Some(n) => n,
                                None => {
                                    i += 1;
                                    continue
                                }
                            };

                            write_value = run_operation(&left, &num, &operation);
                            match instruction.target {
                                Some(target) => wire_to_write = target,
                                None => panic!("No target {:?}", instruction)
                            }
                        },
                        None => {
                            match instruction.target {
                                Some(target) => {
                                    wire_to_write = target;
                                    write_value += left;
                                },
                                None => panic!("No target {:?}", instruction)
                            }
                        }
                    }

                    if wire_to_write != "" {
                        wires.insert(wire_to_write, write_value);
                        processed_lines.push(i);
                    }
                }
            },
            None => {
                match instruction.operation {
                    Some(operation) => {
                        if instruction.right == None {
                            panic!("No right {:?}", instruction);
                        }
                        let right = instruction.right.unwrap();
                        let num: u16 = match parse_or_get_from_map(&num_regex, right, &wires) {
                            Some(n) => n,
                            None => {
                                i += 1;
                                continue
                            }
                        };
                        let temp = 0u16;
                        let write_value = run_operation(&temp, &num, &operation);
                        match instruction.target {
                            Some(target) => {
                                wires.insert(target, write_value);
                                processed_lines.push(i);
                            },
                            None => panic!("No target {:?}", instruction)
                        }
                    },
                    None => panic!("Cannot find operation for {:?}", instruction)
                }
            }
        };
        i += 1;
        if processed_lines.len() == instructions.len() {
            break;
        }
    }
    if wires.contains_key("a") {
        println!("{:?}", wires.get("a").unwrap());
    }
}
