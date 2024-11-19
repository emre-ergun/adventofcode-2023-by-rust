#![feature(iter_map_windows)]

use std::fs;

fn main() {
    let result = calculate_sum_of_part_numbers("./puzzle-input.txt");
    println!("Result: {result}");
}

fn calculate_sum_of_part_numbers(path: &str) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map_windows(|[a, b, c]| {
            1
        })
        .sum()
}
