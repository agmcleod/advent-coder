use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("weights.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let mut weights: Vec<usize> = Vec::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }

        weights.push(line.parse().ok().expect("nope"));
    }

    let weight_per_container = weights.iter().fold(0, |sum, &w| sum + w) / 3;

    let mut start_index = weights.len() - 1;
    let mut index = weights.len() - 1;
    let mut collected_indexes: Vec<usize> = vec![index];

    loop {
        let collected_sum = collected_indexes.iter().map(|i| weights[*i]).fold(0, |sum, w| sum + w);
        if collected_sum < weight_per_container {
            index -= 1;
            collected_indexes.push(index);
        } else if collected_sum > weight_per_container {
            start_index -= 1;
            index = start_index;
            collected_indexes.clear();
            collected_indexes.push(index);
        } else {

        }
    }

    println!("{:?}", weight_per_container);
}
