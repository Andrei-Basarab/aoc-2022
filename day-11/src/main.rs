//! --- Day 11: Monkey in the Middle ---

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
struct Test {
    divisible: u128,
    if_true: u128,
    if_false: u128,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Mul,
}

impl Operator {
    fn from_char(value: char) -> Operator {
        match value {
            '+' => Operator::Sum,
            '*' => Operator::Mul,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    value: u128,
    old: bool,
}

#[derive(Debug)]
struct Monkey {
    id: u128,
    starting_items: Vec<u128>,
    operation: Operation,
    test: Test,
    inspect_counter: u128,
}

impl Default for Monkey {
    fn default() -> Monkey {
        Monkey {
            id: 0,
            starting_items: Vec::new(),
            operation: Operation {
                operator: Operator::Sum,
                value: 0,
                old: false,
            },
            test: Test {
                divisible: 0,
                if_true: 0,
                if_false: 0,
            },
            inspect_counter: 0,
        }
    }
}

#[derive(Debug)]
struct MonkeyGang {
    monkey: Vec<Monkey>,
    divide_by_3: bool,
}

impl Default for MonkeyGang {
    fn default() -> MonkeyGang {
        MonkeyGang {
            monkey: Vec::new(),
            divide_by_3: false,
        }
    }
}

fn parse_monkey_note(monkey_note: String) -> Monkey {
    let mut monkey = Monkey::default();

    for line in monkey_note.lines() {
        if line.starts_with("Monkey ") {
            monkey.id = line
                .strip_prefix("Monkey ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse::<u128>()
                .unwrap();
        } else if line.starts_with("  Starting items: ") {
            monkey.starting_items = line
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect();
        } else if line.starts_with("  Operation: new = old ") {
            let temp: Vec<String> = line
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();
            let operator = Operator::from_char(temp[0].chars().next().unwrap());
            let operand_parse = temp[1].parse::<u128>();

            if operand_parse.is_err() && temp[1] == "old" {
                monkey.operation = Operation {
                    old: true,
                    operator: operator,
                    value: 0,
                };
            } else if operand_parse.is_ok() {
                monkey.operation = Operation {
                    old: false,
                    operator: operator,
                    value: operand_parse.unwrap(),
                };
            }
        } else if line.starts_with("  Test: divisible by ") {
            monkey.test.divisible = line
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse::<u128>()
                .unwrap();
        } else if line.starts_with("    If true: throw to monkey ") {
            monkey.test.if_true = line
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse::<u128>()
                .unwrap();
        } else if line.starts_with("    If false: throw to monkey ") {
            monkey.test.if_false = line
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse::<u128>()
                .unwrap();
        }
    }

    monkey
}

fn parse_monkey_notes(file_content: String) -> MonkeyGang {
    let monkey_notes = file_content.split("\r\n\r\n");
    let mut monkey_gang: MonkeyGang = MonkeyGang::default();

    for monkey_note in monkey_notes {
        monkey_gang
            .monkey
            .push(parse_monkey_note(monkey_note.to_string()));
    }

    monkey_gang
}

fn monkey_business_process(mut monkey_gang: MonkeyGang) -> MonkeyGang {
    let mut lcm: u128 = 1;

    // Find LCM (Least common multiple) - simplified due to prime numbers
    for i in 0..monkey_gang.monkey.len() {
        lcm *= monkey_gang.monkey[i].test.divisible
    }

    // Iterate each Monkey
    for i in 0..monkey_gang.monkey.len() {
        // Iterate each item Monkey is holding
        for j in 0..monkey_gang.monkey[i].starting_items.len() {
            let second_operand: u128;
            let mut new_worry_level: u128 = 0;

            // Monkey inspects an element
            let item = monkey_gang.monkey[i].starting_items[j];

            // Increment the inspect counter
            monkey_gang.monkey[i].inspect_counter += 1;

            // Decide whether to use old or specific value
            if monkey_gang.monkey[i].operation.old == false {
                second_operand = monkey_gang.monkey[i].operation.value;
            } else {
                second_operand = item;
            }

            // Decide on the operation
            if monkey_gang.monkey[i].operation.operator == Operator::Mul {
                new_worry_level = item * second_operand;
            } else if monkey_gang.monkey[i].operation.operator == Operator::Sum {
                new_worry_level = item + second_operand;
            }

            // Boring procedure
            if monkey_gang.divide_by_3 == true {
                new_worry_level = new_worry_level / 3;
            } else {
                new_worry_level = new_worry_level % lcm;
            }

            // Test procedure
            let next_monkey: usize;

            if (new_worry_level % monkey_gang.monkey[i].test.divisible) == 0 {
                next_monkey = monkey_gang.monkey[i].test.if_true as usize;
            } else {
                next_monkey = monkey_gang.monkey[i].test.if_false as usize;
            }

            // Throw to next Monkey
            monkey_gang.monkey[next_monkey]
                .starting_items
                .push(new_worry_level);
        }

        // Clear whatever item the current Monkey had
        monkey_gang.monkey[i].starting_items.clear();
    }

    monkey_gang
}

fn monkey_business(file_content: String, iter: usize, divide_by_3: bool) -> u128 {
    let mut monkey_gang = parse_monkey_notes(file_content);
    monkey_gang.divide_by_3 = divide_by_3;

    // Monkeys play N rounds
    for _ in 0..iter {
        monkey_gang = monkey_business_process(monkey_gang);
    }

    // Find 2 most active Monkeys
    let mut inspect_vec: Vec<u128> = Vec::new();

    for i in 0..monkey_gang.monkey.len() {
        inspect_vec.push(monkey_gang.monkey[i].inspect_counter);
    }

    inspect_vec.sort();
    let top_1 = inspect_vec.pop().unwrap();
    let top_2 = inspect_vec.pop().unwrap();

    top_1 * top_2
}

fn solve_part_one(file_content: String) -> u128 {
    monkey_business(file_content, 20, true)
}

fn solve_part_two(file_content: String) -> u128 {
    monkey_business(file_content, 10000, false)
}

fn main() {
    let file_content = read_input_file();

    // --- Part One ---
    let result = solve_part_one(file_content.clone());
    println!(
        "Figure out which monkeys to chase by counting how many items they inspect over 20 rounds. \
        What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans? Answer: [{}]",
        result
    );

    // --- Part Two ---
    let result = solve_part_two(file_content.clone());

    println!(
        "Starting again from the initial state in your puzzle input, \
        what is the level of monkey business after 10000 rounds? Answer: [{}]",
        result
    );
}
