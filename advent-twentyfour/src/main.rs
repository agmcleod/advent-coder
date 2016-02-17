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

    let mut index = weights.len() - 1;
    let mut collected: Vec<usize> = vec![weights[index]];
    let mut containers: Vec<Vec<usize>> = vec![Vec::new(), Vec::new(), Vec::new()];

    let mut current_container_index = 0;

    let mut weights_copy = weights.clone();

    loop {
        let sum = collected.iter().fold(0, |s, &w| { s + w });
        if sum < weight_per_container {
            collected.push(weights_copy[index]);
            weights_copy.remove(index);
            index = 0;
        } else if sum > weight_per_container {
            weights_copy.insert(collected.pop().unwrap(), 0);
            weights_copy = weights.clone();
            index += 1;
        } else {
            index = 0;
            let mut container = containers.get_mut(current_container_index).unwrap();
            for weight in collected.iter() {
                container.push(weight.clone());
            }
            current_container_index += 1;
            if current_container_index == 2 {
                let sum = weights_copy.iter().fold(0, |s, &w| { s + w });
                if sum != weight_per_container {
                    panic!("did not subtract correctly.");
                }
            }
        }
    }

    println!("{:?}", weight_per_container);
}
