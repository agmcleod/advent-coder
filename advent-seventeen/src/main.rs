use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

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

    let numbers = text.split("\n").map(|s| s.parse().ok().expect("nope")).collect::<Vec<usize>>();
}
