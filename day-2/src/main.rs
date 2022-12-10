//! --- Day 2: Rock Paper Scissors ---

use std::fs;

fn main() {
    let file_path = "./input/input.txt";

    println!("Input File: {}\n", file_path);

    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", file_content);
}
