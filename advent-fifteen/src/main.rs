extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::cmp::max;
use regex::Regex;

fn make_signed_int(value: &str) -> i32 {
    match value.parse() {
        Ok(n) => n,
        Err(e) => panic!("{:?}", e)
    }
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("ingredients.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let regex = Regex::new(r"-?\d+").unwrap();
    let mut ingredient_sets: Vec<[i32; 5]> = Vec::new();

    for line in text.split("\n").filter(|&l| l != "").collect::<Vec<&str>>().iter() {
        let mut ingredient_set = [0 as i32; 5];
        let mut i = 0;
        for num in regex.captures_iter(line) {
            ingredient_set[i] = make_signed_int(num.at(0).unwrap());
            i += 1;
        }

        ingredient_sets.push(ingredient_set);
    }

    let mut scores: Vec<i32> = Vec::new();
    for sugar_ts in (1i32..101i32) {
        for sprinkle_ts in (1i32..(101i32 - sugar_ts)) {
            for candy_ts in (1i32..(101i32 - sugar_ts - sprinkle_ts)) {
                for chocolate_ts in (1i32..(101i32 - sugar_ts - sprinkle_ts - candy_ts)) {
                    let sugar_set = &ingredient_sets[0];
                    let sprinkles_set = &ingredient_sets[1];
                    let candy_set = &ingredient_sets[2];
                    let chocolate_set = &ingredient_sets[3];

                    if (sugar_ts * sugar_set[4] + sprinkle_ts * sprinkles_set[4] + candy_ts * candy_set[4] + chocolate_ts * chocolate_set[4] == 500) {
                        scores.push(
                            max(sugar_ts * sugar_set[0] + sprinkle_ts * sprinkles_set[0] + candy_ts * candy_set[0] + chocolate_ts * chocolate_set[0], 0) *
                            max(sugar_ts * sugar_set[1] + sprinkle_ts * sprinkles_set[1] + candy_ts * candy_set[1] + chocolate_ts * chocolate_set[1], 0) *
                            max(sugar_ts * sugar_set[2] + sprinkle_ts * sprinkles_set[2] + candy_ts * candy_set[2] + chocolate_ts * chocolate_set[2], 0) *
                            max(sugar_ts * sugar_set[3] + sprinkle_ts * sprinkles_set[3] + candy_ts * candy_set[3] + chocolate_ts * chocolate_set[3], 0)
                        );
                    }
                }
            }
        }
    }

    scores.sort();
    println!("{:?}", scores[scores.len() - 1]);
}
