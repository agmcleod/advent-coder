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
            location_collection.push(Location::new(String::from(target_location.clone()), distance));
        } else {
            let mut location_collection: Vec<Location> = Vec::new();
            let location: Location = Location::new(String::from(target_location.clone()), distance);
            location_collection.push(location);
            location_distances.insert(String::from(root_location.clone()), location_collection);
        }

        if location_distances.contains_key(&target_location) {
            let mut location_collection = location_distances.get_mut(&target_location).unwrap();
            location_collection.push(Location::new(root_location, distance));
        } else {
            let mut location_collection: Vec<Location> = Vec::new();
            let location: Location = Location::new(root_location, distance);
            location_collection.push(location);
            location_distances.insert(target_location, location_collection);
        }
    }

    let mut distances: Vec<usize> = Vec::new();

    for (key, locations) in location_distances.iter() {
        // try each permutation as a start point for this starting location key
        for location in locations.iter() {
            let mut traversed_locations: Vec<String> = Vec::new();
            let mut distance = location.distance;
            let mut route = key.clone();
            route.push_str(" -> ");
            route.push_str(location.location_key.clone().as_ref());
            traversed_locations.push(String::from(key.clone()));
            traversed_locations.push(String::from(location.location_key.clone()));
            let mut location_key = &location.location_key;
            loop {
                let locations = location_distances.get(location_key).unwrap();
                for l in locations.iter() {
                    if !traversed_locations.contains(&l.location_key) {
                        distance += l.distance;
                        route.push_str(" -> ");
                        route.push_str(l.location_key.clone().as_ref());
                        traversed_locations.push(String::from(l.location_key.clone()));
                        location_key = &l.location_key;
                        break;
                    }
                }
                if traversed_locations.len() == location_distances.keys().len() {
                    println!("{:?} = {:?}\n", route, distance);
                    distances.push(distance);
                    break;
                }
            }
        }
    }
    distances.sort();
    println!("{:?}", distances[0]);
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
