use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn should_be_lit(lights: &Vec<Vec<&str>>, row: &i16, cell: &i16, is_on: bool) -> bool {
    let mut lights_on_count = 0;

    let cells = [
        (row - 1, cell - 1), (row - 1, *cell), (row - 1, cell + 1),
        (*row, cell-1), (*row, cell + 1),
        (row + 1, cell - 1), (row + 1, *cell), (row + 1, cell + 1)
    ];

    let len = lights.len() as i16;

    for coords in cells.iter() {
        let &(r, c) = coords;
        if r < 0 || c < 0 || r >= len || c >= len {
            continue
        } else if lights[r as usize][c as usize] == "#" {
            println!("{}, {}, {:?}", row, cell, lights[*row as usize]);
            lights_on_count += 1;
        }
    }

    if is_on {
        lights_on_count == 2 || lights_on_count == 3
    } else {
        lights_on_count == 3
    }
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("lights.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn run_iteration(lights: &mut Vec<Vec<&str>>) {
    let cloned = lights.clone();
    let mut r = 0i16;
    for row in lights.iter_mut() {
        let mut c = 0i16;
        for cell in row.iter_mut() {
            if should_be_lit(&cloned, &r, &c, *cell == "#") {
                *cell = "#";
            } else {
                *cell = ".";
            }
            c += 1;
        }
        r += 1;
    }
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let mut lights = Vec::<Vec<&str>>::new();
    let mut copy = Vec::<Vec<&str>>::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }
        lights.push(line.split("").filter(|c| *c != "").collect::<Vec<&str>>());
        copy.push(line.split("").filter(|c| *c != "").collect::<Vec<&str>>());
    }

    let mut i = 0;
    loop {
        run_iteration(&mut lights);
        println!("{:?}", lights);
        i += 1;
        if i >= 1 {
            break;
        }
    }

    let mut light_count = 0;
    for row in lights.iter() {
        for cell in row.iter() {
            if *cell == "#" {
                light_count += 1;
            }
        }
    }

    println!("{:?}", light_count);
}
