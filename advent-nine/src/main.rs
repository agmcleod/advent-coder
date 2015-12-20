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

fn append_location(
    location_distances: &mut HashMap<String, Vec<Location>>,
    location_one: &String,
    location_two: &String,
    distance: usize) {
    if location_distances.contains_key(location_one) {
        let mut location_collection = location_distances.get_mut(location_one).unwrap();
        location_collection.push(Location::new(String::from(location_two.clone()), distance));
    } else {
        let mut location_collection: Vec<Location> = Vec::new();
        let location: Location = Location::new(String::from(location_two.clone()), distance);
        location_collection.push(location);
        location_distances.insert(String::from(location_one.clone()), location_collection);
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

        append_location(&mut location_distances, &root_location, &target_location, distance);
        append_location(&mut location_distances, &target_location, &root_location, distance);
    }

    let mut distances: Vec<usize> = Vec::new();
    for (key, locations) in location_distances.iter() {
        let mut traversed_locations: Vec<String> = Vec::new();
        traversed_locations.push(String::from(key.as_ref()).clone());
        distances.extend(get_distances_from_locations(&mut traversed_locations, &location_distances, &key).iter().cloned());
    }

    distances.sort();

    println!("{:?}", distances[distances.len()-1]);
}

fn get_distances_from_locations(traversed_locations: &mut Vec<String>, location_distances: &HashMap<String, Vec<Location>>, key: &String) -> Vec<usize> {
    let mut distances = Vec::new();

    let locations = location_distances.get(key).unwrap();
    for location in locations {
        if !traversed_locations.contains(&location.location_key) {
            if traversed_locations.len() == locations.len() {
                distances.push(location.distance)
            } else {
                let len = traversed_locations.len().clone();
                traversed_locations.push(String::from(location.location_key.as_ref()));
                let child_distances = get_distances_from_locations(traversed_locations, location_distances, &location.location_key);
                let distance = location.distance;
                for dist in child_distances.iter() {
                    distances.push(distance + dist);
                }
                loop {
                    if traversed_locations.len() == len {
                        break;
                    }
                    traversed_locations.pop();
                }
            }
        }
    }

    distances
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
