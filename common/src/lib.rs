use std::fs;
use std::path::PathBuf;

pub static INPUT_FILE: &str = "./input/input.txt";

pub fn read_input_file() -> String {
    let file_path = PathBuf::from(INPUT_FILE);
    println!(
        "Input File: {:?}\n",
        fs::canonicalize(&file_path).expect("Error")
    );
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[derive(Debug, PartialEq)]
pub struct Puzzle<'a> {
    question: &'a str,
    answer: u32,
}

pub fn get_puzzle_part(question: &str, answer: u32) -> Puzzle {
    Puzzle { question, answer }
}

pub fn print_puzzle(puzzle_name: &str, puzzle: [Puzzle; 2]) {
    let tab = " ".repeat(4);
    let part = ["One", "Two"];

    println!("{}", puzzle_name);
    println!();

    for i in 0..2 {
        println!("{}Part {}:", tab.repeat(1), part[i]);
        println!("{}{}{}", tab.repeat(2), "Question: ", puzzle[i].question);
        println!("{}{}{}", tab.repeat(2), "  Answer: ", puzzle[i].answer);
        println!();
    }
}

/// ================ Unit Tests ================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nominal_get_puzzle_part() {
        let puzzle = Puzzle {
            question: "test",
            answer: 10,
        };
        let result = get_puzzle_part("test", 10);
        assert_eq!(result, puzzle);
    }
}
