use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn next_index(containers: &Vec<Vec<usize>>, index: &usize) -> usize {
    let mut next_i = index - 1;

    loop {
        let mut index_used = false;
        for container in containers.iter() {
            if container.contains(&next_i) {
                index_used = true;
                break
            }
        }

        if index_used {
            next_i -= 1;
        } else {
            break
        }
    }

    next_i
}

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
    let mut containers: Vec<Vec<usize>> = vec![Vec::new(), Vec::new(), Vec::new()];

    let mut current_container_index = 0;

    let mut weights_copy = weights.clone();

    loop {
        let collected_sum = collected_indexes.iter().map(|i| weights[*i]).fold(0, |sum, w| sum + w);
        if collected_sum < weight_per_container {
            index = next_index(&containers, &index);
            collected_indexes.push(index);
        } else if collected_sum > weight_per_container {
            start_index = next_index(&containers, &start_index);
            index = start_index;
            collected_indexes.clear();
            collected_indexes.push(index);
        } else {
            for index in collected_indexes.iter() {
                containers.get_mut(current_container_index).unwrap().push(index.clone());
            }
            collected_indexes.clear();
            current_container_index += 1;

            if current_container_index == 2 {

            }
        }
    }

    println!("{:?}", weight_per_container);
}
