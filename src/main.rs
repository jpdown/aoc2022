extern crate core;

use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    day1::part1();
    day1::part2();
    day2::part1();
    day2::part2();
    day3::part1();
    day3::part2();
}

fn read_file(file: &str) -> String {
    let mut path = "inputs/".to_owned();
    path.push_str(file);
    let contents = fs::read_to_string(path);
    return contents.unwrap();
}