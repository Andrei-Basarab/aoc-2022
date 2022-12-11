//! --- Day 5: Supply Stacks ---

use regex::Regex;
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
struct Stack {
    number: u32,
    stack: Vec<char>,
}

impl Stack {
    fn new(value: u32) -> Self {
        Stack {
            stack: Vec::new(),
            number: value,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.stack.last()
    }
}

fn get_stack_index(stacks: &Vec<Stack>, number: u32) -> usize {
    let mut found_index = 0;

    for stack in stacks {
        if number == stack.number {
            break;
        }
        found_index += 1;
    }

    found_index
}

fn parse_crates(vec: Vec<String>) -> Vec<Stack> {
    let mut stack_primitive: Vec<String> = Vec::new();
    let mut stacks: Vec<Stack> = Vec::new();
    let mut stack_index = 0;

    // Convert table text into vector of string stacks
    for index in (1..((&vec[0]).len())).step_by(4) {
        let mut stack_str = String::new();

        for line in &vec {
            stack_str.push(line.chars().nth(index).unwrap());
        }

        stack_primitive.push(stack_str);
    }

    // Convert stack_primitive to fully functional stack
    for stack_string in stack_primitive {
        for index in 0..(stack_string.len()) {
            if index == 0 {
                stacks.push(Stack::new(
                    stack_string
                        .chars()
                        .rev()
                        .nth(index)
                        .unwrap()
                        .to_digit(10)
                        .unwrap(),
                ));
            } else {
                let char = stack_string.chars().rev().nth(index).unwrap();
                if char != ' ' {
                    stacks[stack_index].stack.push(char);
                }
            }
        }
        stack_index += 1;
    }

    stacks
}

#[derive(Debug)]
struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

fn parse_instructions(vec: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in vec {
        let capture = re.captures(&line).unwrap();
        instructions.push(Instruction {
            count: (&capture[1]).parse::<u32>().unwrap(),
            from: (&capture[2]).parse::<u32>().unwrap(),
            to: (&capture[3]).parse::<u32>().unwrap(),
        });
    }

    instructions
}

fn execute_rearrangement_procedure(
    mut stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
    multi_crates: bool,
) -> Vec<Stack> {
    let mut buffer_stack = Stack::new(0);

    for instruction in instructions {
        let from_index = get_stack_index(&stacks, instruction.from);
        let to_index = get_stack_index(&stacks, instruction.to);

        if multi_crates == false {
            for _ in 0..instruction.count {
                let payload = stacks[from_index].stack.pop().unwrap();
                stacks[to_index].stack.push(payload);
            }
        } else {
            for _ in 0..instruction.count {
                let payload = stacks[from_index].stack.pop().unwrap();
                buffer_stack.stack.push(payload);
            }
            for _ in 0..instruction.count {
                let payload = buffer_stack.stack.pop().unwrap();
                stacks[to_index].stack.push(payload);
            }
        }
    }

    stacks
}

fn get_top_crates(stacks: &Vec<Stack>) -> String {
    let mut string = String::new();

    for stack in stacks {
        string.push(*stack.peek().unwrap());
    }

    string
}

fn semantic_parsing(file_content: String) -> (Vec<Stack>, Vec<Instruction>) {
    let lines = file_content.lines();
    let mut vec_lines_crates: Vec<String> = Vec::new();
    let mut vec_lines_instructions: Vec<String> = Vec::new();

    // Extract lines with crates
    for line in lines.clone() {
        vec_lines_crates.push(line.to_string());

        if line.starts_with(" 1") {
            break;
        }
    }

    // Extract lines with instructions
    for line in lines.clone() {
        if line.starts_with("move") {
            vec_lines_instructions.push(line.to_string());
        }
    }

    (
        parse_crates(vec_lines_crates),
        parse_instructions(vec_lines_instructions),
    )
}

fn solve_part_one(file_content: String) -> String {
    let mut stacks: Vec<Stack>;
    let instructions: Vec<Instruction>;

    (stacks, instructions) = semantic_parsing(file_content);

    // Execute rearrangement (CrateMover 9000)
    stacks = execute_rearrangement_procedure(stacks, instructions, false);

    get_top_crates(&stacks)
}

fn solve_part_two(file_content: String) -> String {
    let mut stacks: Vec<Stack>;
    let instructions: Vec<Instruction>;

    (stacks, instructions) = semantic_parsing(file_content);

    // Execute rearrangement (CrateMover 9001)
    stacks = execute_rearrangement_procedure(stacks, instructions, true);

    get_top_crates(&stacks)
}

fn main() {
    let file_content = read_input_file();

    // --- Part One ---
    let result = solve_part_one(file_content.clone());
    println!(
        "After the rearrangement procedure completes, what crate ends up on top of each stack? Answer: [{}]",
        result
    );

    // --- Part Two ---
    let result = solve_part_two(file_content.clone());

    println!(
        "After the rearrangement procedure completes, what crate ends up on top of each stack? Answer: [{}]",
        result
    );
}
