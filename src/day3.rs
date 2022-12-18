use std::collections::HashMap;

pub fn part1() {
    println!("Starting day3 part1");

    let rucksacks = parse_rucksacks();
    let sum = get_all_rucksacks_sum(rucksacks);
    println!("{}", sum);
}

pub fn part2() {
    println!("Starting day3 part1");

    let rucksacks = parse_rucksacks();
    let sum = get_group_sum(rucksacks);
    println!("{}", sum);
}

fn parse_rucksacks() -> Vec<[HashMap<char, u32>; 2]> {
    let mut rucksacks = Vec::new();
    let file_contents = super::read_file("day3.txt");

    for line in file_contents.lines() {
        if line.is_empty() {
            continue;
        }

        let contents = parse_rucksack_contents(line);
        rucksacks.push([contents.0, contents.1]);
    }

    return rucksacks;
}

fn parse_rucksack_contents(contents: &str) -> (HashMap<char, u32>, HashMap<char, u32>) {
    let mut compartment_one = HashMap::new();
    let mut compartment_two = HashMap::new();
    let chars = contents.char_indices();
    let compartment_size = contents.len() / 2;

    for (i, char) in chars {
        if i < compartment_size {
            compartment_one.insert(char, map_char_to_priority(char));
        } else {
            compartment_two.insert(char, map_char_to_priority(char));
        }
    }

    return (compartment_one, compartment_two);
}

fn get_all_rucksacks_sum(rucksacks: Vec<[HashMap<char, u32>; 2]>) -> u32 {
    let mut sum = 0;

    for rucksack in rucksacks {
        sum += get_matching_compartment_sum(&rucksack[0], &rucksack[1]);
    }

    return sum;
}

fn get_matching_compartment_sum(compartment_one: &HashMap<char, u32>, compartment_two: &HashMap<char, u32>) -> u32 {
    let mut sum = 0;

    for char in compartment_one.keys() {
        let compartment_two_result = compartment_two.get(char);
        sum += match compartment_two_result {
            Some(..) => compartment_two_result.unwrap(),
            None => &0,
        }
    }

    return sum;
}

fn get_group_sum(rucksacks: Vec<[HashMap<char, u32>; 2]>) -> u32 {
    let mut sum = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let group_type = get_group_item_type([&rucksacks[i], &rucksacks[i+1], &rucksacks[i+2]]);
        sum += map_char_to_priority(group_type);
    }

    return sum;
}

fn get_group_item_type(rucksacks: [&[HashMap<char, u32>; 2]; 3]) -> char {
    // Only one char will be contained in all 3 rucksacks

    for char in rucksacks[0][0].keys() {
        if (rucksacks[1][0].contains_key(char) || rucksacks[1][1].contains_key(char))
            && (rucksacks[2][0].contains_key(char) || rucksacks[2][1].contains_key(char)) {
            return *char;
        }
    }

    for char in rucksacks[0][1].keys() {
        if (rucksacks[1][0].contains_key(char) || rucksacks[1][1].contains_key(char))
            && (rucksacks[2][0].contains_key(char) || rucksacks[2][1].contains_key(char)) {
            return *char;
        }
    }

    return '?';
}

fn map_char_to_priority(input: char) -> u32 {
    // a-z -> 1-26
    // A-Z -> 27-52
    let ascii_value = input as u32;

    if ascii_value >= 65 && ascii_value <= 90 {
        return ascii_value - 64 + 26;
    } else if ascii_value >= 97 && ascii_value <= 122 {
        return ascii_value - 96;
    }
    return 0;
}