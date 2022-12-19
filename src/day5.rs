pub fn part1() {
    println!("Starting day5 part1");
    let (mut stacks, steps) = read_puzzle_input();
    run_sim_single_moves(&mut stacks, steps);
    println!("{}", get_top_of_stacks(stacks));
}

pub fn part2() {
    println!("Starting day5 part2");
    let (mut stacks, steps) = read_puzzle_input();
    run_sim_multi_moves(&mut stacks, steps);
    println!("{}", get_top_of_stacks(stacks));
}

struct Step {
    num_to_move: usize,
    source: usize,
    dest: usize,
}

fn run_sim_single_moves(stacks: &mut Vec<Vec<char>>, steps: Vec<Step>) {
    for step in steps {
        for _ in 0..step.num_to_move {
            let moved_crate = stacks[step.source].pop().expect("uh oh");
            stacks[step.dest].push(moved_crate);
        }
    }
}

fn run_sim_multi_moves(stacks: &mut Vec<Vec<char>>, steps: Vec<Step>) {
    let mut curr_movement = Vec::new();
    for step in steps {
        for _ in 0..step.num_to_move {
            let moved_crate = stacks[step.source].pop().expect("uh oh");
            curr_movement.push(moved_crate);
        }
        curr_movement.reverse();
        stacks[step.dest].append(&mut curr_movement);
        curr_movement.clear();
    }
}

fn get_top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    let mut tops = String::new();

    for stack in stacks {
        tops.push(*stack.last().unwrap_or(&' '));
    }

    return tops;
}

fn read_puzzle_input() -> (Vec<Vec<char>>, Vec<Step>) {
    let file_contents = super::read_file("day5.txt");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut steps: Vec<Step> = Vec::new();

    for line in file_contents.lines() {
        if line.contains("[") {
            read_stack_state(line, &mut stacks);
        } else if line.starts_with("move") {
            read_step(line, &mut steps);
        }
    }

    reverse_stacks(&mut stacks);

    return (stacks, steps);
}

fn reverse_stacks(stacks: &mut Vec<Vec<char>>) {
    for stack in stacks {
        stack.reverse();
    }
}

fn read_stack_state(line: &str, stacks: &mut Vec<Vec<char>>) {
    let splits = line.split(" ");
    let mut skip = 0;
    let mut index = 0;

    // if split contains [], get the middle char
    // if split does not contain [], we have to skip 4

    for split in splits {
        if skip == 4 {
            skip = 0;
            index += 1;
        }

        if split.contains("[") {
            skip = 0;
            expand_stacks(index + 1, stacks);
            stacks[index].push(split.chars().nth(1).expect("invalid stack"));
            index += 1;
        } else {
            skip += 1;
        }
    }
}

fn read_step(line: &str, steps: &mut Vec<Step>) {
    let step_input: Vec<&str> = line.split(" ").collect();
    let new_step = Step {
        num_to_move: step_input[1].parse().expect("bad move input"),
        source: step_input[3].parse::<usize>().expect("bad move input") - 1,
        dest: step_input[5].parse::<usize>().expect("bad move input") - 1,
    };

    steps.push(new_step);
}

fn expand_stacks(size: usize, stacks: &mut Vec<Vec<char>>) {
    let len = stacks.len();
    if len < size {
        for _ in len..size {
            stacks.push(Vec::new());
        }
    }
}