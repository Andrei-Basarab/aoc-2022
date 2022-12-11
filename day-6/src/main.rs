//! --- Day 6: Tuning Trouble ---

use core::str::Chars;
use std::fs;
use std::path::PathBuf;

static INPUT_FILE: &str = "./input/input.txt";

fn read_input_file() -> String {
    let file_path = PathBuf::from(INPUT_FILE);
    println!(
        "Input File: {:?}\n",
        fs::canonicalize(&file_path).expect("Error")
    );
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn check_unique_chars(char_vec: Vec<char>) -> bool {
    let mut unique_chars = true;

    'outer: for i in 0..char_vec.len() {
        for j in (i + 1)..char_vec.len() {
            if char_vec[i] == char_vec[j] {
                unique_chars = false;
                break 'outer;
            }
        }
    }

    unique_chars
}

fn marker_seeker(char_iter: Chars, char_count: usize) -> u32 {
    let char_vec: Vec<char> = char_iter.collect();
    let mut result: u32 = 0;

    for i in (char_count - 1)..(char_vec.len()) {
        let mut marker_vec: Vec<char> = Vec::new();

        for j in 0..char_count {
            marker_vec.push(char_vec[i + j + 1 - char_count]);
        }

        let marker_found = check_unique_chars(marker_vec);

        if marker_found == true {
            result = (i as u32) + 1;
            break;
        }
    }

    result
}

fn solve_part_one(file_content: String) -> u32 {
    // Search start-of-packet marker
    let result = marker_seeker(file_content.chars(), 4);

    result
}

fn solve_part_two(file_content: String) -> u32 {
    // Search start-of-message marker
    let result = marker_seeker(file_content.chars(), 14);

    result
}

fn main() {
    let file_content = read_input_file();

    // --- Part One ---
    let result = solve_part_one(file_content.clone());
    println!(
        "How many characters need to be processed before the first start-of-packet marker is detected? Answer: [{}]",
        result
    );

    // --- Part Two ---
    let result = solve_part_two(file_content.clone());

    println!(
        "How many characters need to be processed before the first start-of-message marker is detected? Answer: [{}]",
        result
    );
}
