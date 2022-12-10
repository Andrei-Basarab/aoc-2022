//! --- Day 2: Rock Paper Scissors ---

use std::fs;

#[derive(Debug, Copy)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl Clone for RockPaperScissor {
    fn clone(&self) -> RockPaperScissor {
        let value = *self;
        value
    }
}

impl RockPaperScissor {
    fn from_char(value: char) -> RockPaperScissor {
        match value {
            'A' => RockPaperScissor::Rock,
            'B' => RockPaperScissor::Paper,
            'C' => RockPaperScissor::Scissor,
            'X' => RockPaperScissor::Rock,
            'Y' => RockPaperScissor::Paper,
            'Z' => RockPaperScissor::Scissor,
            _ => panic!("Unknown value: {}", value),
        }
    }

    fn to_score(value: RockPaperScissor) -> u32 {
        match value {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
        }
    }
}

#[derive(Debug)]
enum RockPaperScissorResult {
    Loss,
    Draw,
    Win,
}

impl RockPaperScissorResult {
    fn to_score(value: RockPaperScissorResult) -> u32 {
        match value {
            RockPaperScissorResult::Loss => 0,
            RockPaperScissorResult::Draw => 3,
            RockPaperScissorResult::Win => 6,
        }
    }
}

fn line_to_char_set(line: &str) -> (char, char) {
    let strategy_set: Vec<&str> = line.split_whitespace().collect();

    (
        strategy_set[0].chars().nth(0).unwrap(),
        strategy_set[1].chars().nth(0).unwrap(),
    )
}

fn get_rock_paper_scissor_score(opponent: RockPaperScissor, me: RockPaperScissor) -> u32 {
    let selection_score = RockPaperScissor::to_score(me);
    let round_score = RockPaperScissorResult::to_score(rock_paper_scissor(opponent, me));
    let total_score = selection_score + round_score;

    total_score
}

fn rock_paper_scissor(opponent: RockPaperScissor, me: RockPaperScissor) -> RockPaperScissorResult {
    match (opponent, me) {
        (RockPaperScissor::Rock, RockPaperScissor::Rock) => RockPaperScissorResult::Draw,
        (RockPaperScissor::Rock, RockPaperScissor::Paper) => RockPaperScissorResult::Win,
        (RockPaperScissor::Rock, RockPaperScissor::Scissor) => RockPaperScissorResult::Loss,
        (RockPaperScissor::Paper, RockPaperScissor::Rock) => RockPaperScissorResult::Loss,
        (RockPaperScissor::Paper, RockPaperScissor::Paper) => RockPaperScissorResult::Draw,
        (RockPaperScissor::Paper, RockPaperScissor::Scissor) => RockPaperScissorResult::Win,
        (RockPaperScissor::Scissor, RockPaperScissor::Rock) => RockPaperScissorResult::Win,
        (RockPaperScissor::Scissor, RockPaperScissor::Paper) => RockPaperScissorResult::Loss,
        (RockPaperScissor::Scissor, RockPaperScissor::Scissor) => RockPaperScissorResult::Draw,
    }
}

fn main() {
    let file_path = "./input/input.txt";

    println!("Input File: {}\n", file_path);

    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split_lines = file_content.lines();
    let mut total_score = 0;

    for line in split_lines {
        let char_set = line_to_char_set(line);

        total_score += get_rock_paper_scissor_score(
            RockPaperScissor::from_char(char_set.0),
            RockPaperScissor::from_char(char_set.1),
        );
    }

    println!("What would your total score be if everything goes exactly according to your strategy guide? Answer: [{}]", total_score);
}
