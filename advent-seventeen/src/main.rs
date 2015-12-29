extern crate time;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

const VOLUME_TARGET: usize = 150;

fn add_one_to_flags(use_flags: &mut Vec<usize>) {
    for flag in use_flags.iter_mut() {
        if *flag == 1 {
            *flag = 0;
        } else if *flag == 0 {
            *flag = 1;
            break;
        } else {
            panic!("Invalid number: {}", flag);
        }
    }
}

fn generate_power_sets(sizes: &Vec<usize>) -> Vec<Vec<usize>> {
    let len = sizes.len();
    let mut use_flags = Vec::<usize>::new();
    let mut sets: Vec<Vec<usize>> = Vec::new();
    for i in 0..len {
        use_flags.push(0);
    }
    loop {
        add_one_to_flags(&mut use_flags);
        let mut set = Vec::<usize>::new();
        let mut index = 0;
        for flag in use_flags.iter() {
            if *flag == 1 {
                set.push(index);
            }
            index += 1;
        }

        sets.push(set);

        if !use_flags.contains(&0) {
            break;
        }
    }

    let sets = sets.into_iter().rev().collect::<Vec<_>>();

    sets
}

fn make_number(value: &str) -> usize {
    value.parse().ok().expect("nope")
}

pub fn process_sizes(sizes: &Vec<usize>) -> usize {
    let mut unique_count = 0;

    let mut sets = generate_power_sets(&sizes);
    sets.sort_by(|a, b| b.len().cmp(&a.len()));

    for set in sets.iter() {
        if set.len() == 1 {
            break
        }
        if set.iter().fold(0, |acc, &v| acc + sizes[v]) == VOLUME_TARGET {
            unique_count += 1;
        }
    }

    unique_count
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("sizes.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let sizes = text.split("\n").filter(|s| *s != "").map(|s| make_number(s)).collect::<Vec<usize>>();
    let start = time::now().to_timespec();
    println!("{:?}", process_sizes(&sizes));
    println!("{:?}", time::now().to_timespec() - start);
}
