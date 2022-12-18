//! --- Day 8: Treetop Tree House ---

use std::cmp::max;
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
struct Tree {
    x: usize,
    y: usize,
    height: u8,
    visible: bool,
    scenic_score: u32,
}

#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<Tree>>,
}

impl Default for Forest {
    fn default() -> Forest {
        Forest { grid: Vec::new() }
    }
}

fn parse_input_file(file_content: String) -> Forest {
    let mut forest = Forest::default();
    let lines = file_content.lines();
    let mut line_index = 0;
    let mut column_index = 0;

    for line in lines {
        forest.grid.push(Vec::new());

        for char in line.chars() {
            forest.grid[line_index].push(Tree {
                x: column_index,
                y: line_index,
                height: char.to_digit(10).unwrap() as u8,
                visible: false,
                scenic_score: 0,
            });
            column_index += 1;
        }

        column_index = 0;
        line_index += 1;
    }

    forest
}

fn is_tree_on_edge(tree: &Tree, max_x: usize, max_y: usize) -> bool {
    if (tree.x == 0) || (tree.y == 0) || (tree.x == (max_x - 1)) || (tree.y == (max_y - 1)) {
        return true;
    } else {
        return false;
    }
}

fn check_visible_trees(forest: &mut Forest) {
    let row_len = (&(forest.grid)).len();
    let column_len = (&(forest.grid))[0].len();

    for i in 0..row_len {
        for j in 0..column_len {
            if is_tree_on_edge(&(forest.grid[i][j]), row_len, column_len) {
                forest.grid[i][j].visible = true;
            } else {
                let mut max_height = 0;
                let current_height = forest.grid[i][j].height;

                // Check Up
                for k in 0..i {
                    max_height = max(max_height, forest.grid[k][j].height);
                }

                if current_height > max_height {
                    forest.grid[i][j].visible = true;
                    continue;
                } else {
                    max_height = 0;
                }

                // Check Right
                for k in (j + 1)..column_len {
                    max_height = max(max_height, forest.grid[i][k].height);
                }

                if current_height > max_height {
                    forest.grid[i][j].visible = true;
                    continue;
                } else {
                    max_height = 0;
                }

                // Check Down
                for k in (i + 1)..row_len {
                    max_height = max(max_height, forest.grid[k][j].height);
                }

                if current_height > max_height {
                    forest.grid[i][j].visible = true;
                    continue;
                } else {
                    max_height = 0;
                }

                // Check Left
                for k in 0..j {
                    max_height = max(max_height, forest.grid[i][k].height);
                }

                if current_height > max_height {
                    forest.grid[i][j].visible = true;
                    continue;
                } else {
                    // max_height = 0;
                }
            }
        }
    }
}

fn count_visible_trees(forest: &Forest) -> u32 {
    let mut tree_counter = 0;
    let row_len = (&(forest.grid)).len();
    let column_len = (&(forest.grid))[0].len();

    for i in 0..row_len {
        for j in 0..column_len {
            if forest.grid[i][j].visible == true {
                tree_counter += 1;
            }
        }
    }

    tree_counter
}

#[allow(dead_code)]
fn print_trees_visibility(forest: &Forest) {
    for row in &(forest.grid) {
        for tree in row {
            if tree.visible == true {
                print!(".");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

fn compute_scenic_scores(forest: &mut Forest) {
    let row_len = (&(forest.grid)).len();
    let column_len = (&(forest.grid))[0].len();

    for i in 0..row_len {
        for j in 0..column_len {
            if (is_tree_on_edge(&(forest.grid[i][j]), row_len, column_len) == false)
                && (forest.grid[i][j].visible == true)
            {
                let mut scenic_score: Vec<u32> = Vec::new();
                let mut tree_count = 0;
                let current_height = forest.grid[i][j].height;

                // Check Up
                for k in (0..i).rev() {
                    tree_count += 1;
                    if current_height <= forest.grid[k][j].height {
                        break;
                    }
                }
                scenic_score.push(tree_count);
                tree_count = 0;

                // Check Right
                for k in (j + 1)..column_len {
                    tree_count += 1;
                    if current_height <= forest.grid[i][k].height {
                        break;
                    }
                }
                scenic_score.push(tree_count);
                tree_count = 0;

                // Check Down
                for k in (i + 1)..row_len {
                    tree_count += 1;
                    if current_height <= forest.grid[k][j].height {
                        break;
                    }
                }
                scenic_score.push(tree_count);
                tree_count = 0;

                // Check Left
                for k in (0..j).rev() {
                    tree_count += 1;
                    if current_height <= forest.grid[i][k].height {
                        break;
                    }
                }
                scenic_score.push(tree_count);
                // tree_count = 0;

                forest.grid[i][j].scenic_score = scenic_score.iter().product();
            }
        }
    }
}

fn get_max_scenic_score(forest: &Forest) -> u32 {
    let row_len = (&(forest.grid)).len();
    let column_len = (&(forest.grid))[0].len();
    let mut max_scenic_score = 0;

    for i in 0..row_len {
        for j in 0..column_len {
            max_scenic_score = max(max_scenic_score, forest.grid[i][j].scenic_score);
        }
    }

    max_scenic_score
}

fn solve_part_one(file_content: String) -> u32 {
    let mut forest = parse_input_file(file_content);
    check_visible_trees(&mut forest);
    count_visible_trees(&forest)
}

fn solve_part_two(file_content: String) -> u32 {
    let mut forest = parse_input_file(file_content);
    check_visible_trees(&mut forest);
    compute_scenic_scores(&mut forest);
    get_max_scenic_score(&forest)
}

fn main() {
    let file_content = read_input_file();

    // --- Part One ---
    let result = solve_part_one(file_content.clone());
    println!(
        "Consider your map; how many trees are visible from outside the grid? Answer: [{}]",
        result
    );

    // --- Part Two ---
    let result = solve_part_two(file_content.clone());

    println!(
        "Consider each tree on your map. \
        What is the highest scenic score possible for any tree? Answer: [{}]",
        result
    );
}
