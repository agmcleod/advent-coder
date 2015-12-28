use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn next_size(sizes: &Vec<usize>, indexes: &mut Vec<usize>, unique_count: &mut usize, amount_filled: &mut usize) {

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

    let sizes = text.split("\n").map(|s| s.parse().ok().expect("nope")).collect::<Vec<usize>>();
    let mut unique_count = 0;

    let mut indexes: Vec<usize> = Vec::new();
    let mut amount_filled = 0;
    next_size(&sizes, &mut indexes, &mut unique_count, &mut amount_filled);

    println!("{:?}", unique_count);
}
