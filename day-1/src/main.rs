//! --- Day 1: Calorie Counting ---

use std::fs;

fn main() {
    let file_path = "./input/input.txt";
    let two_newlines = "\r\n\r\n";

    println!("Input File: {}\n", file_path);

    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split_groups = file_content.split(two_newlines);

    let mut calories_group_max = 0;
    let mut vec = Vec::new();

    // Iterate over calorie groups
    for group in split_groups {
        let split_lines = group.lines();
        let mut calories = 0;

        // Iterate over each calorie in a group
        for line in split_lines {
            calories += line.parse::<i32>().unwrap();
        }

        vec.push(calories);

        // Find Max
        if calories > calories_group_max {
            calories_group_max = calories;
        }
    }

    // Sort elfs with highest calories
    vec.sort();

    let mut top_3_elves = 0;

    // Find how much calories do the 3 top elves have together
    for i in 0..3 {
        top_3_elves += vec[vec.len() - 1 - i];
    }

    // Answers
    println!("Find the Elf carrying the most Calories. How many total Calories is that Elf carrying? Answer: [{}]", calories_group_max);
    println!("Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total? Answer: [{}]", top_3_elves);
}
