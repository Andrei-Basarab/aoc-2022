//! --- Day 3: Rucksack Reorganization ---

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

fn get_common_char(first_str: &str, second_str: &str) -> char {
    assert!(
        first_str.len() == second_str.len(),
        "String are not of the same length!"
    );
    let mut found_char: char = 'a';

    for char in first_str.chars() {
        if second_str.find(char) != None {
            found_char = char;
        }
    }

    found_char
}

fn convert_char_to_value(char: char) -> u8 {
    let result: u8;

    if char.is_alphabetic() {
        if char.is_lowercase() {
            result = (char as u8) - 96;
        } else {
            result = (char as u8) - 38;
        }
    } else {
        panic!("Char is not alphabetic!")
    }

    result
}

fn main() {
    let file_content = read_input_file();
    let lines = file_content.lines();
    let mut sum: u32 = 0;

    // --- Part One ---
    for line in lines.clone() {
        let first_half = &line.to_string()[0..(line.len() / 2)];
        let second_half = &line.to_string()[(line.len() / 2)..line.len()];

        let common_char = get_common_char(&first_half, &second_half);
        let value = convert_char_to_value(common_char);
        sum += value as u32;
    }

    println!("Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types? Answer: [{}]", sum);

    // --- Part Two ---
    sum = 0;
    let mut str_arr: [&str; 3] = [""; 3];
    let mut str_vec: Vec<[&str; 3]> = Vec::new();
    let mut str_index = 0;

    for line in lines {
        str_arr[str_index] = line;
        str_index += 1;

        if str_index == 3 {
            str_index = 0;
            str_vec.push(str_arr);
        }
    }

    for group in str_vec {
        let mut common_char: char = 'a';

        for char in group[0].chars() {
            if (group[1].find(char) != None) && (group[2].find(char) != None) {
                common_char = char;
            }
        }
        let value = convert_char_to_value(common_char);
        sum += value as u32;
    }

    println!("Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types? Answer: [{}]", sum);
}
