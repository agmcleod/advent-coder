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

fn run_condition(property: &str, value: &usize, expected_value: &usize) -> bool {
    if property == "trees" || property == "cats" {
        value > expected_value
    } else if property == "goldfish" || property == "pomeranians" {
        value < expected_value
    } else {
        value == expected_value
    }
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    // get rid of those troublesome characters
    let text = text.replace(":", "").replace(",", "");

    let mut sues: Vec<HashMap<&str, usize>> = Vec::new();
    let property_names = vec!["children", "cats", "samoyeds", "pomeranians", "akitas", "vizslas", "goldfish", "trees", "cars", "perfumes"];
    let mut criteria = HashMap::new();
    criteria.insert("children", 3);
    criteria.insert("cats", 7);
    criteria.insert("samoyeds", 2);
    criteria.insert("pomeranians", 3);
    criteria.insert("akitas", 0);
    criteria.insert("vizslas", 0);
    criteria.insert("goldfish", 5);
    criteria.insert("trees", 3);
    criteria.insert("cars", 2);
    criteria.insert("perfumes", 1);

    let mut sue_num = 1;
    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        let words = line.split(" ").collect::<Vec<&str>>();

        let mut properties: HashMap<&str, usize> = HashMap::new();
        properties.insert("index", sue_num);

        let mut index = 0;
        for word in words.iter() {
            for property_name in property_names.iter() {
                if word == property_name {
                    properties.insert(property_name, make_number(words[index + 1]));
                }
            }
            index += 1;
        }
        sue_num += 1;
        sues.push(properties);
    }

    let sues = sues
        .iter()
        .filter(|sue| {
            sue.iter().filter(|&(property, value)| *property != "index").fold(true, |acc, (&property, value)| {
                acc && run_condition(property, value, criteria.get(property).unwrap())
            })
        })
        .collect::<Vec<_>>();

    println!("{:?}", sues);
}
