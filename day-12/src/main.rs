extern crate common;
mod puzzle;

use crate::common::{get_puzzle_part, print_puzzle, read_input_file};
use crate::puzzle::{
    puzzle::solve_part_one, puzzle::solve_part_two, puzzle::PUZZLE_NAME, puzzle::QUESTION_ONE,
    puzzle::QUESTION_TWO,
};

fn main() {
    let file_content = read_input_file();

    print_puzzle(
        PUZZLE_NAME.to_string(),
        [
            get_puzzle_part(QUESTION_ONE.to_string(), solve_part_one(&file_content)),
            get_puzzle_part(QUESTION_TWO.to_string(), solve_part_two(&file_content)),
        ],
    );
}
