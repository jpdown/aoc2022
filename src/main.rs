use std::fs;

mod day1;

fn main() {
    day1::part1();
    day1::part2();
}

fn read_file(file: &str) -> String {
    let mut path = "inputs/".to_owned();
    path.push_str(file);
    let contents = fs::read_to_string(path);
    return contents.unwrap();
}