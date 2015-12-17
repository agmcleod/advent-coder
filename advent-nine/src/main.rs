use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct Location {
    location_key: String,
    distance: usize
}

impl Location {
    pub fn new(location_key: String, distance: usize) -> Location {
        Location{ location_key: location_key, distance: distance }
    }
}

fn parse_lines<'a>(lines: &Vec<&str>) {
    let mut location_distances: HashMap<String, Vec<Location>> = HashMap::new();

    for line in lines.iter() {
        if *line == "" {
            continue
        }
        let pieces: Vec<&str> = line.split(" ").collect();

        let root_location = String::from(pieces[0]);
        let target_location = String::from(pieces[2]);
        let distance: usize = pieces[4].parse().ok().expect("nope");

        if location_distances.contains_key(&root_location) {
            let mut location_collection = location_distances.get_mut(&root_location).unwrap();
            location_collection.push(Location::new(target_location, distance));
        } else {
            let mut location_collection: Vec<Location> = Vec::new();
            let location: Location = Location::new(target_location, distance);
            location_collection.push(location);
            location_distances.insert(root_location, location_collection);
        }
    }

    for (key, locations) in location_distances.iter() {
        println!("{:?} {:?}", key, locations);
    }
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("distances.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(err) => panic!("Could not read file {:?}", err)
    };

    let lines: Vec<&str> = text.split("\n").collect();
    parse_lines(&lines);
}
