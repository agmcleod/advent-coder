use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("input.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

#[derive(Debug)]
enum InstructionType {
    HLF, TPL, INC, JMP, JIE, JIO
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    value: Option<i32>,
    register: Option<String>
}

impl Instruction {
    fn new(instruction_type: InstructionType, value: Option<i32>, register: Option<String>) -> Instruction {
        Instruction{ instruction_type: instruction_type, value: value, register: register }
    }
}

fn num_from_str(st: &String) -> i32 {
    match st.parse() {
        Ok(n) => n,
        Err(e) => panic!("{:?} {:?}", e, st)
    }
}

fn reference_for_register_opt<'a>(a: &'a mut usize, b: &'a mut usize, register: &String) -> &'a mut usize {
    match register.as_ref() {
        "a" => a,
        "b" => b,
        _ => panic!("Invalid register value: {:?}", register)
    }
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let mut a = 0;
    // part 2
    let mut a = 1;
    let mut b = 0;

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }
        let words = line.split(" ").collect::<Vec<&str>>();
        let arg1 = words[1].replace(",", "");
        let value = if words.len() > 2 {
            let arg2 = String::from(words[2]);
            Some(num_from_str(&arg2))
        } else {
            None
        };

        let register = match arg1.as_ref() {
            "a" => Some(String::from("a")),
            "b" => Some(String::from("b")),
            _ => None
        };
        let instruction = match words[0] {
            "hlf" => Instruction::new(InstructionType::HLF, None, register),
            "tpl" => Instruction::new(InstructionType::TPL, None, register),
            "inc" => Instruction::new(InstructionType::INC, None, register),
            "jmp" => Instruction::new(InstructionType::JMP, Some(num_from_str(&arg1)), None),
            "jie" => Instruction::new(InstructionType::JIE, value, register),
            "jio" => Instruction::new(InstructionType::JIO, value, register),
            _ => panic!("Invalid register: {:?}", words)
        };

        instructions.push(instruction);
    }

    let mut instruction_index = 0i32;

    loop {
        let instruction = instructions.get(instruction_index as usize).unwrap();
        match instruction.instruction_type {
            InstructionType::HLF => {
                let register_key = instruction.register.as_ref().unwrap();
                let mut register = reference_for_register_opt(&mut a, &mut b, register_key);
                *register /= 2;
                instruction_index += 1;
            },
            InstructionType::TPL => {
                let register_key = instruction.register.as_ref().unwrap();
                let mut register = reference_for_register_opt(&mut a, &mut b, register_key);
                *register *= 3;
                instruction_index += 1;
            },
            InstructionType::INC => {
                let register_key = instruction.register.as_ref().unwrap();
                let mut register = reference_for_register_opt(&mut a, &mut b, register_key);
                *register += 1;
                instruction_index += 1;
            },
            InstructionType::JMP => {
                let value = instruction.value.as_ref().unwrap();
                instruction_index += *value;
                continue
            },
            InstructionType::JIE => {
                let value = instruction.value.as_ref().unwrap();
                let register_key = instruction.register.as_ref().unwrap();
                let register = reference_for_register_opt(&mut a, &mut b, register_key);
                if *register % 2 == 0 {
                    instruction_index += *value;
                } else {
                    instruction_index += 1;
                }
            },
            InstructionType::JIO => {
                let value = instruction.value.as_ref().unwrap();
                let register_key = instruction.register.as_ref().unwrap();
                let register = reference_for_register_opt(&mut a, &mut b, register_key);
                if *register == 1 {
                    instruction_index += *value;
                } else {
                    instruction_index += 1;
                }
            }
        }

        if instruction_index as usize >= instructions.len() {
            break
        }
    }

    println!("{:?}", b);
}
