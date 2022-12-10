//! --- Day 4: Camp Cleanup ---

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

#[derive(Debug)]
struct SectionRange {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct ElfPair {
    elf_one: SectionRange,
    elf_two: SectionRange,
}

fn convert_line_to_elf_pair(line: &str) -> ElfPair {
    let mut pair = line.split(",");
    let mut elf_one_range = pair.next().unwrap().split("-");
    let mut elf_two_range = pair.next().unwrap().split("-");

    let elf_pair: ElfPair = ElfPair {
        elf_one: SectionRange {
            start: elf_one_range.next().unwrap().parse::<u32>().unwrap(),
            end: elf_one_range.next().unwrap().parse::<u32>().unwrap(),
        },
        elf_two: SectionRange {
            start: elf_two_range.next().unwrap().parse::<u32>().unwrap(),
            end: elf_two_range.next().unwrap().parse::<u32>().unwrap(),
        },
    };

    elf_pair
}

fn elves_section_range_full_overlap(elf_pair: ElfPair) -> bool {
    let overlap: bool;

    if (elf_pair.elf_one.start <= elf_pair.elf_two.start)
        && (elf_pair.elf_one.end >= elf_pair.elf_two.end)
    {
        overlap = true;
    } else if (elf_pair.elf_one.start >= elf_pair.elf_two.start)
        && (elf_pair.elf_one.end <= elf_pair.elf_two.end)
    {
        overlap = true;
    } else {
        overlap = false;
    }

    overlap
}

fn elves_section_range_partial_overlap(elf_pair: ElfPair) -> bool {
    let overlap: bool;

    if elf_pair.elf_one.end < elf_pair.elf_two.start {
        overlap = false;
    } else if elf_pair.elf_one.start > elf_pair.elf_two.end {
        overlap = false;
    } else {
        overlap = true;
    }

    overlap
}

fn solve_part_one(file_content: String) -> u32 {
    let lines = file_content.lines();
    let mut sum = 0;

    for line in lines {
        let overlap = elves_section_range_full_overlap(convert_line_to_elf_pair(line));
        sum += overlap as u32;
    }

    sum
}

fn solve_part_two(file_content: String) -> u32 {
    let lines = file_content.lines();
    let mut sum = 0;

    for line in lines {
        let overlap = elves_section_range_partial_overlap(convert_line_to_elf_pair(line));
        sum += overlap as u32;
    }

    sum
}

fn main() {
    let file_content = read_input_file();

    // --- Part One ---
    let result = solve_part_one(file_content.clone());
    println!(
        "In how many assignment pairs does one range fully contain the other? Answer: [{}]",
        result
    );

    // --- Part Two ---
    let result = solve_part_two(file_content.clone());

    println!(
        "In how many assignment pairs do the ranges overlap? Answer: [{}]",
        result
    );
}
