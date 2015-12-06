extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use regex::Regex;

pub fn contains_two_of_a_pair(word: &str) -> bool {
    let mut contains_repeated_pair = false;
    let mut end_of_pair_index = 1;

    let characters = get_characters(word);
    for character in characters.iter() {
        if end_of_pair_index < characters.len() {
            let pair = [*character, characters[end_of_pair_index]];
            let mut index = 0;
            for test_character in characters.iter() {
                if index > end_of_pair_index && index + 1 < characters.len() {
                    if *test_character == pair[0] && characters[index + 1] == pair[1] {
                        contains_repeated_pair = true;
                    }
                }
                index += 1;
            }
        }
        end_of_pair_index += 1;
    }

    contains_repeated_pair
}

pub fn contains_a_skip_letter(word: &str) -> bool {
    let mut contains_skip = false;

    let mut index = 0;
    let characters = get_characters(word);
    let len = characters.len();
    for character in characters.iter() {
        if index + 2 < len && characters[index + 2] == *character {
            contains_skip = true;
            break;
        }
        index += 1;
    }

    contains_skip
}

fn get_characters(word: &str) -> Vec<&str> {
    let words: Vec<&str> = word.split("").collect();
    words.into_iter().filter(|&s| s != "").collect::<Vec<&str>>()
}

fn get_vowel_count(word: &str, regex: &Regex) -> usize {
    let mut vowel_count = 0;
    let characters = get_characters(word);
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
    let characters = get_characters(word);
    let mut last_character = "";
    let mut has_double = false;
    for character in characters.iter() {
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
        let words: Vec<&str> = text.split("\n").collect();
        let mut nice_word_count = 0;
        let bad_words = ["ab", "cd", "pq", "xy"];
        let vowel_regex = Regex::new(r"a|e|i|o|u").unwrap();

        for word in words.iter() {
            if !has_bad_words(bad_words, word) && get_vowel_count(word, &vowel_regex) >= 3 && has_double_letters(word) {
                nice_word_count += 1;
            }
        }

        println!("{:?}", nice_word_count);
        nice_word_count = 0;

        for word in words.iter() {
            if contains_two_of_a_pair(word) && contains_a_skip_letter(word) {
                nice_word_count += 1;
            }
        }

        println!("{:?}", nice_word_count);
    },
    Err(error) => panic!("{:?}", error)
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_two_of_a_pair() {
        let word = "aaab";
        assert_eq!(contains_two_of_a_pair(word), false);

        let word = "aabaa";
        assert_eq!(contains_two_of_a_pair(word), true);

        let word = "xyaxy";
        assert_eq!(contains_two_of_a_pair(word), true);
    }

    #[test]
    fn test_contains_a_skip_letter() {
        let word = "abcd";
        assert_eq!(contains_a_skip_letter(word), false);

        let word = "abad";
        assert_eq!(contains_a_skip_letter(word), true);
    }
}
