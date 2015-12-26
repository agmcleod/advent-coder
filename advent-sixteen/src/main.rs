use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn make_number(s: &str) -> usize {
    s.parse().ok().expect("nope")
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("suelist.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let mut sues: Vec<HashMap<&str, usize>> = Vec::new();
    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        let words = line.split(" ").collect::<Vec<&str>>();
        
    }
}
