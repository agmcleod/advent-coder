pub fn has_three_characters_in_a_row(characters: &Vec<u8>) -> bool {
    let mut index = 0;
    let mut result = false;
    loop {
        if characters[index + 2] > characters[index + 1] && characters[index + 1] > characters[index] &&
            characters[index + 1] - characters[index] == 1 && characters[index + 2] - characters[index] == 2 {
            result = true;
        }
        index += 1;
        if index + 2 >= characters.len() {
            break;
        }
    }

    result
}

pub fn has_two_same_letter_pairs(characters: &Vec<u8>) -> bool {
    let mut index = 0;
    let mut pair_count = 0;
    loop {
        if index + 1 < characters.len() && characters[index] == characters[index + 1] {
            pair_count += 1;
            index += 2;
        }
        index += 1;
        if index >= characters.len() {
            break;
        }
    }

    pair_count >= 2
}

fn next_sequence(characters: &mut Vec<u8>, index: usize) {
    let mut next_one = false;
    {
        let mut character: &mut u8 = characters.get_mut(index).unwrap();
        *character += 1;
        if *character > b'z' {
            *character = b'a';
            next_one = true;
        }
    }
    if next_one {
        if index == 0 {
            let len = characters.len();
            next_sequence(characters, len - 1);
        } else {
            next_sequence(characters, index - 1);
        }
    }
}

fn print_characters(characters: &Vec<u8>) {
    println!("{:?}", String::from_utf8(characters.clone()).unwrap());
}

fn validate_sequence(characters: &Vec<u8>, bad_letters: &Vec<u8>, alphabet: &Vec<u8>) -> bool {
    let mut valid = true;

    for bad_letter in bad_letters.iter() {
        if characters.contains(bad_letter) {
            valid = false;
            break;
        }
    }

    if valid {
        has_three_characters_in_a_row(characters) && has_two_same_letter_pairs(characters)
    } else {
        false
    }
}

fn main() {
    let alphabet: Vec<u8> = (b'a'..b'{').collect();
    let bad_letters: Vec<u8> = vec![b'l', b'o', b'i'];

    let mut characters: Vec<u8> = "hxbxwxba".as_bytes().iter().cloned().collect();

    print_characters(&characters);
    let mut iteration_count = 0;
    loop {
        let len = characters.len();
        next_sequence(&mut characters, len - 1);
        if validate_sequence(&characters, &bad_letters, &alphabet) {
            iteration_count += 1;
            if iteration_count >= 2 {
                break;
            }
        }
    }
    print_characters(&characters);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_three_characters_in_a_row() {
        let characters: Vec<u8> = "abchuyw".as_bytes().iter().cloned().collect();
        assert_eq!(has_three_characters_in_a_row(&characters), true);

        let characters: Vec<u8> = "abdeiok".as_bytes().iter().cloned().collect();
        assert_eq!(has_three_characters_in_a_row(&characters), false);
    }

    #[test]
    fn test_has_two_same_letter_pairs() {
        let characters: Vec<u8> = "abchuyw".as_bytes().iter().cloned().collect();
        assert_eq!(has_two_same_letter_pairs(&characters), false);

        let characters: Vec<u8> = "aacbb".as_bytes().iter().cloned().collect();
        assert_eq!(has_two_same_letter_pairs(&characters), true);

        let characters: Vec<u8> = "aaab".as_bytes().iter().cloned().collect();
        assert_eq!(has_two_same_letter_pairs(&characters), false);
    }
}
