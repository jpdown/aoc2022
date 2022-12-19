pub fn part1() {
    println!("Starting day4 part1");

    let pairs = read_elf_pairs();
    let num_overlap = get_num_full_overlaps(&pairs);

    println!("{}", num_overlap);
}

pub fn part2() {
    println!("Starting day4 part2");

    let pairs = read_elf_pairs();
    let num_overlap = get_num_overlaps(&pairs);

    println!("{}", num_overlap);
}

struct Elf {
    start: i32,
    end: i32,
}

struct ElfPair(Elf, Elf);

impl ElfPair {
    fn has_full_overlap(&self) -> bool {
       return (self.0.start >= self.1.start && self.0.end <= self.1.end)
        || (self.1.start >= self.0.start && self.1.end <= self.0.end);
    }

    fn has_overlap(&self) -> bool {
        return (self.0.start <= self.1.end && self.0.start >= self.1.start)
            || (self.1.start <= self.0.end && self.1.start >= self.0.start);
    }
}

fn get_num_full_overlaps(pairs: &Vec<ElfPair>) -> i32 {
    let mut num = 0;

    for pair in pairs {
        if pair.has_full_overlap() {
            num += 1;
        }
    }

    return num;
}

fn get_num_overlaps(pairs: &Vec<ElfPair>) -> i32 {
    let mut num = 0;

    for pair in pairs {
        if pair.has_overlap() {
            num += 1;
        }
    }

    return num;
}

fn read_elf_pairs() -> Vec<ElfPair> {
    let file_contents = super::read_file("day4.txt");
    let mut elf_pairs = Vec::new();

    for line in file_contents.lines() {
        if line.is_empty() {
            continue;
        }

        elf_pairs.push(parse_elf_pair(line));
    }

    return elf_pairs;
}

fn parse_elf_pair(line: &str) -> ElfPair {
    let split_elves: Vec<&str> = line.split(",").collect();

    return ElfPair (
        parse_elf(split_elves[0]),
        parse_elf(split_elves[1])
    );
}

fn parse_elf(elf_input: &str) -> Elf {
    let split: Vec<&str> = elf_input.split("-").collect();
    let start: i32 = split[0].parse().expect("invalid elf input");
    let end: i32 = split[1].parse().expect("invalid elf input");
    return Elf {
        start,
        end,
    }
}