use crate::read_file;

pub fn part1() {
    println!("Starting day6 part1");
    println!("{}", find_unique_sequence(read_file("day6.txt"), 4) + 1);
}

pub fn part2() {
    println!("Starting day6 part2");
    println!("{}", find_unique_sequence(read_file("day6.txt"), 14) + 1);
}

fn find_unique_sequence(input: String, num_unique_chars: usize) -> usize {
    let mut found_index = 0;
    let mut char_buffer = vec![' '; num_unique_chars];
    let mut char_to_replace = 0;

    for (i, char) in input.char_indices() {
        found_index = i;
        char_buffer[char_to_replace] = char;
        char_to_replace = (char_to_replace + 1) % num_unique_chars;

        if i > 3 && all_chars_unique(&char_buffer) {
            break;
        }
    }

    return found_index;
}

fn all_chars_unique(buffer: &[char]) -> bool {
    let mut unique = true;

    for i in 0..buffer.len() {
        for j in (i+1)..buffer.len() {
            if buffer[i] == buffer[j] {
                unique = false;
                break;
            }
        }
    }

    return unique;
}