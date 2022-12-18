pub fn part1() {
    println!("Starting day part1");
    let mut elves = get_elf_totals();
    println!("{}", elves[0]);
}

pub fn part2() {
    println!("Starting day part1");
    let mut elves = get_elf_totals();
    let mut top = 0;

    for i in 0..3 {
        top += elves[i];
    }

    println!("{}", top);
}

fn get_elf_totals() -> Vec<i32> {
    let mut elves = vec!(0);
    let file_contents = super::read_file("day1.txt");
    let mut curr_elf = 0;

    for line in file_contents.lines() {
        if line.is_empty() {
            curr_elf += 1;
            elves.push(0);
        } else {
            let cals: i32 = line.parse().expect("non int cals input");
            elves[curr_elf] += cals;
        }
    }

    elves.sort();
    elves.reverse();

    return elves;
}