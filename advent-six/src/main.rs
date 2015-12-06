use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

#[derive(Copy,Clone)]
pub enum LightState {
    On,
    Off,
}

pub enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

type Grid = [[LightState; 1000]; 1000];

fn coords_from_string(string_nums: Vec<&str>) -> Vec<u32> {
    string_nums.iter().map(|&c| {
        let c_as_string = String::from(c);
        match c_as_string.parse() {
            Ok(value) => value,
            Err(err) => panic!("{:?}", err)
        }
    }).collect()
}

fn make_grid() -> Grid {
    [[LightState::Off; 1000]; 1000]
}

fn run_command(command: Command, args: Vec<&str>, grid: &mut Grid) {
    let string_nums: Vec<&str> = args[0].split(",").collect();
    let low_coords: Vec<u32> = coords_from_string(string_nums);
    let string_nums: Vec<&str> = args[2].split(",").collect();
    let high_coords: Vec<u32> = coords_from_string(string_nums);

    for x in low_coords[0]..(high_coords[0] + 1) {
        for y in low_coords[1]..(high_coords[1] + 1) {
            let x = x as usize;
            let y = y as usize;
            match command {
                Command::Toggle => {
                    match grid[x][y] {
                        LightState::Off => {
                            grid[x][y] = LightState::On;
                        },
                        LightState::On => {
                            grid[x][y] = LightState::Off;
                        }
                    }
                },
                Command::TurnOff => {
                    grid[x][y] = LightState::Off;
                },
                Command::TurnOn => {
                    grid[x][y] = LightState::On;
                }
            }
        }
    }
}

fn output_lights_on_count(grid: Grid) {
    let mut count = 0;
    for col in grid.iter() {
        for cell in col.iter() {
            match *cell {
                LightState::On => count += 1,
                _ => continue
            }
        }
    }

    println!("{:?}", count);
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("commands.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    match read_text() {
        Ok(text) => {
            let lines: Vec<&str> = text.split("\n").collect();
            let mut grid = make_grid();
            for line in lines.iter() {
                let words: Vec<&str> = line.split(" ").collect();
                let command: Command;
                let mut args_index = 0;
                match words[0] {
                    "toggle" => {
                        command = Command::Toggle;
                        args_index = 1;
                    },
                    "turn" => {
                        match words[1] {
                            "on" => {
                                command = Command::TurnOn;
                                args_index = 2;
                            },
                            "off" => {
                                command = Command::TurnOff;
                                args_index = 2;
                            },
                            _ => continue
                        }
                    }
                    _ => continue
                }
                let args = words[args_index..(words.len())].to_vec();
                run_command(command, args, &mut grid);
            }

            output_lights_on_count(grid);
        }
        Err(err) => panic!("{:?}", err)
    }
}
