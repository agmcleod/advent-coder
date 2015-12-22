fn process_value(value: &String) -> String {
    let pieces: Vec<&str> = value.split("").collect();
    let mut result = Vec::<String>::new();
    let mut last_character = "";
    let mut count = 1;
    for character in pieces.iter() {
        if *character == "" {
            continue;
        }
        if last_character != "" {
            if last_character != *character {
                result.push(format!("{}{}", count, last_character));
                count = 1;
            } else {
                count += 1;
            }
        }

        last_character = &character;
    }

    result.push(format!("{}{}", count, last_character));
    let result = result.join("");
    result
}

fn main() {
    let mut input = String::from("3113322113");
    for i in 1..51 {
        input = process_value(&input);
    }

    println!("len {:?}", input.len());
}
