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

    weights.reverse();
    let weight_per_container = weights.iter().fold(0, |sum, &w| sum + w) / 3;

    let mut totals = Vec::<usize>::new();

    for i in 0..weights.len() {
        let mut index = i;
        let mut collected: Vec<usize> = Vec::new();
        let mut containers: Vec<Vec<usize>> = vec![Vec::new(), Vec::new(), Vec::new()];

        let mut current_container_index = 0;

        let mut start_index = 0;

        let mut weights_copy = weights.clone();

        loop {
            let sum = collected.iter().fold(0, |s, &w| { s + w });
            if sum < weight_per_container {
                if index >= weights_copy.len() {
                    start_index += 1;
                    if start_index >= weights_copy.len() {
                        break
                    }
                    index = start_index;
                    while collected.len() > 0 {
                        weights_copy.push(collected.pop().unwrap());
                    }

                    weights_copy.sort();
                    weights_copy.reverse();
                }
                collected.push(match weights_copy.get(index) {
                    Some(r) => r.clone(),
                    None => panic!("could not get weights_copy[{:?}] start_index: {:?}", index, start_index)
                });
                weights_copy.remove(index);
            } else if sum > weight_per_container {
                if weights_copy.len() == 0 {
                    weights_copy.push(collected.pop().unwrap());
                } else {
                    weights_copy.insert(0, collected.pop().unwrap());
                }
                index += 1;
            } else {
                index = 0;
                {
                    let mut container = containers.get_mut(current_container_index).unwrap();
                    for weight in collected.iter() {
                        container.push(weight.clone());
                    }
                }
                collected.clear();
                // println!("{:?}", weights_copy);
                current_container_index += 1;
                if current_container_index == 2 {
                    containers.sort_by(|a, b| {
                        a.len().cmp(&b.len())
                    });
                    totals.push(containers.get(1).unwrap().iter().fold(1, |factor, value| factor * value));
                    break
                }
            }
        }
    }

    totals.sort();
    println!("{:?}", totals);
}
