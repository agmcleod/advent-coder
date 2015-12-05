extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use regex::Regex;

fn get_vowel_count(word: &str, regex: &Regex) -> usize {
    let mut vowel_count = 0;
    let characters: Vec<&str> = word.split("").collect();
    for character in characters.iter() {
        if regex.is_match(character) {
            vowel_count += 1;
        }
    }

    vowel_count
}

fn has_bad_words(bad_words: [&str; 4], word: &str) -> bool {
    let mut is_bad = false;
    for bad_word in bad_words.into_iter() {
        if word.contains(bad_word) {
            is_bad = true;
        }
    }
    is_bad
}

fn has_double_letters(word: &str) -> bool {
    let characters: Vec<&str> = word.split("").collect();
    let mut last_character = "";
    let mut has_double = false;
    for character in characters.iter() {
        if *character == "" {
            continue;
        }
        if **character == *last_character {
            has_double = true;
        }
        last_character = character;
    }
    has_double
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("in.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
  match read_text() {
    Ok(text) => {
        let words: Vec<&str> = text.split("\r\n").collect();
        let mut nice_word_count = 0;
        let bad_words = ["ab", "cd", "pq", "xy"];
        let vowel_regex = Regex::new(r"a|e|i|o|u").unwrap();

        for word in words.iter() {
            if !has_bad_words(bad_words, word) && get_vowel_count(word, &vowel_regex) >= 3 && has_double_letters(word) {
                nice_word_count += 1;
            }
        }

        println!("{:?}", nice_word_count);
    },
    Err(error) => panic!("{:?}", error)
  }
}
