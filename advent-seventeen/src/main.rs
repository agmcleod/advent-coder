use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

struct Indexer {
    current_index: usize,
    start_index: usize,
    rollback_index: usize
}

fn make_number(value: &str) -> usize {
    value.parse().ok().expect("nope")
}

fn next_iteration(sizes: &Vec<usize>, indexer: &mut Indexer, unique_count: &mut usize, amount_filled: &mut usize) {
    indexer.rollback_index += 1;
    if indexer.rollback_index >= sizes.len() {
        indexer.start_index += 1;
        indexer.current_index = indexer.start_index.clone();
        indexer.rollback_index = indexer.start_index + 1;
    } else {
        indexer.current_index = indexer.rollback_index.clone();
        *amount_filled = sizes[indexer.start_index];
        next_size(sizes, indexer, unique_count, amount_filled);
    }
}

fn next_size(sizes: &Vec<usize>, indexer: &mut Indexer, unique_count: &mut usize, amount_filled: &mut usize) {
    if indexer.current_index == sizes.len() {
        next_iteration(sizes, indexer, unique_count, amount_filled);
    } else if *amount_filled + sizes[indexer.current_index] > 150 {
        indexer.current_index += 1;
        next_size(&sizes, indexer, unique_count, amount_filled);
    } else {
        if *amount_filled + sizes[indexer.current_index] == 150 {
            *unique_count += 1;
            indexer.current_index += 1;
            next_size(&sizes, indexer, unique_count, amount_filled);
        } else {
            *amount_filled += sizes[indexer.current_index];
            indexer.current_index += 1;
            next_size(&sizes, indexer, unique_count, amount_filled);
        }
    }
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

    let mut indexer = Indexer{current_index: 0, start_index: 0, rollback_index: 1};

    let sizes = text.split("\n").filter(|s| *s != "").map(|s| make_number(s)).collect::<Vec<usize>>();
    let mut unique_count = 0;

    loop {
        let mut amount_filled = 0;
        next_size(&sizes, &mut indexer, &mut unique_count, &mut amount_filled);
        if indexer.start_index == sizes.len() {
            break
        }
    }


    println!("{:?}", unique_count);
}
