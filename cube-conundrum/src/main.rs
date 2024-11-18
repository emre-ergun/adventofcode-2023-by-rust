#![allow(dead_code, unused_variables)]

use std::fs;

const MAX_RED_CUBES: u8 = 12;
const MAX_GREEN_CUBES: u8 = 13;
const MAX_BLUE_CUBES: u8 = 14;

fn main() {
    let result = calculate_sum_of_ids("./puzzle-input.txt");
    println!("Sum of IDs: {result}");
}

fn calculate_sum_of_ids(path: &str) -> u32 {
    fs::read_to_string(path)
        .expect("Read error")
        .lines()
        .map(|s| {
            let sections = s.split(':').collect::<Vec<&str>>();
            let game_sections = sections[0].split_whitespace().collect::<Vec<&str>>();
            let game_id = game_sections[1].parse::<u32>().unwrap();

            let cube_sections = sections[1]
                .split(&[';', ',', ' '][..])
                .filter(|s| !s.is_empty())
                .map(|s| s.trim())
                .collect::<Vec<&str>>();

            let result = cube_sections.chunks(2).all(|x| {
                let cube_number = x[0].parse::<u8>().unwrap();
                let cube_color = x[1];

                match cube_color {
                    "red" => {
                        if cube_number > MAX_RED_CUBES {
                            return false;
                        }
                    }
                    "green" => {
                        if cube_number > MAX_GREEN_CUBES {
                            return false;
                        }
                    }
                    "blue" => {
                        if cube_number > MAX_BLUE_CUBES {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }

                true
            });

            if result {
                game_id
            } else {
                0
            }
        })
        .sum()
}
