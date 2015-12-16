extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use regex::Regex;

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("in.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(err) => panic!("Could not read file {:?}", err)
    };

    let lines: Vec<&str> = text.split("\n").collect();
    let mut unparsed_count = 0;
    let mut parsed_count = 0;

    let regex = Regex::new(r#"(\\)|(")"#).unwrap();
    for line in lines.iter() {
        if *line == "" {
            continue;
        }
        unparsed_count += line.len();
        let substituted_text = format!(r#""{}""#, regex.replace_all(line, ".."));
        println!("{} {}", line, substituted_text);
        parsed_count += substituted_text.len();
    }

    let amt = parsed_count - unparsed_count;
    println!("{} {} {}", unparsed_count, parsed_count, amt);
}
