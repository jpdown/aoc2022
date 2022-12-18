use crate::day2::Move::{PAPER, ROCK, SCISSORS};
use crate::day2::Strategy::{DRAW, LOSE, WIN};

pub fn part1() {
    println!("Starting day2 part1");
    let inputs = get_inputs();

    let opponent_moves = parse_opponent_moves(inputs.0);
    let player_moves = parse_player_moves(inputs.1);

    println!("{}", get_total_score(opponent_moves, player_moves));
}

pub fn part2() {
    println!("Starting day2 part2");
    let inputs = get_inputs();

    let opponent_moves = parse_opponent_moves(inputs.0);
    let player_strategies = parse_player_strategies(inputs.1);
    let player_moves = get_moves_from_strategies(player_strategies, &opponent_moves);

    println!("{}", get_total_score(opponent_moves, player_moves));
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Clone, Copy)]
enum Strategy {
    WIN,
    DRAW,
    LOSE,
}

fn get_inputs() -> (Vec<char>, Vec<char>) {
    let mut opponent = Vec::new();
    let mut player = Vec::new();
    let file_contents = super::read_file("day2.txt");

    for line in file_contents.lines() {
        if (line.is_empty()) {
            break;
        }
        opponent.push(line.chars().nth(0).expect("invalid input"));
        player.push(line.chars().nth(2).expect("invalid input"));
    }

    return (opponent, player);
}

fn get_total_score(opponent: Vec<Move>, player: Vec<Move>) -> i32 {
    let mut score = 0;
    assert_eq!(opponent.len(), player.len());

    for i in 0..opponent.len() {
        score += calculate_score(opponent[i], player[i]);
    }

    return score;
}

fn calculate_score(opponent: Move, player: Move) -> i32 {
    // Start with move score
    let mut score = player as i32;

    // Add score for win/draw
    if opponent == player {
        score += 3;
    } else if player_won(opponent, player) {
        score += 6;
    }

    return score;
}

fn player_won(opponent: Move, player: Move) -> bool {
    let player_int = player as i32;
    let opponent_int = opponent as i32;
    return true_mod(player_int - opponent_int, 3) == 1;
}

fn true_mod(a: i32, b: i32) -> i32 {
    return ((a % b) + b) % b;
}

fn parse_opponent_moves(inputs: Vec<char>) -> Vec<Move> {
    let mut moves = Vec::new();

    for input in inputs {
        moves.push(map_opponent_to_move(input));
    }

    return moves;
}

fn parse_player_moves(inputs: Vec<char>) -> Vec<Move> {
    let mut moves = Vec::new();

    for input in inputs {
        moves.push(map_player_to_move(input));
    }

    return moves;
}

fn parse_player_strategies(inputs: Vec<char>) -> Vec<Strategy> {
    let mut strategies = Vec::new();

    for input in inputs {
        strategies.push(map_player_to_strategy(input));
    }

    return strategies;
}

fn get_moves_from_strategies(strategies: Vec<Strategy>, opponent_moves: &Vec<Move>) -> Vec<Move> {
    let mut moves = Vec::new();

    assert_eq!(strategies.len(), opponent_moves.len());

    for i in 0..strategies.len() {
        moves.push(map_strategy_to_move(strategies[i], opponent_moves[i]));
    }

    return moves;
}

fn map_opponent_to_move(input: char) -> Move {
    return match input {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => panic!("bad opponent input"),
    }
}

fn map_player_to_move(input: char) -> Move {
    return match input {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSORS,
        _ => panic!("bad player input"),
    }
}

fn map_player_to_strategy(input: char) -> Strategy {
    return match input {
        'X' => LOSE,
        'Y' => DRAW,
        'Z' => WIN,
        _ => panic!("bad player input")
    }
}

fn map_strategy_to_move(strategy: Strategy, opponent_move: Move) -> Move {
    return match strategy {
        WIN => get_winning_move(opponent_move),
        DRAW => opponent_move,
        LOSE => get_losing_move(opponent_move),
    }
}

fn get_winning_move(opponent_move: Move) -> Move {
    return match opponent_move {
        ROCK => PAPER,
        PAPER => SCISSORS,
        SCISSORS => ROCK,
    }
}

fn get_losing_move(opponent_move: Move) -> Move {
    return match opponent_move {
        ROCK => SCISSORS,
        PAPER => ROCK,
        SCISSORS => PAPER,
    }
}

#[test]
fn test_player_won() {
    assert_eq!(true, player_won(SCISSORS, ROCK));
    assert_eq!(true, player_won(ROCK, PAPER));
    assert_eq!(true, player_won(PAPER, SCISSORS));
    assert_eq!(false, player_won(SCISSORS, PAPER));
    assert_eq!(false, player_won(ROCK, SCISSORS));
    assert_eq!(false, player_won(PAPER, ROCK));
}