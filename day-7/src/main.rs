//! --- Day 7: No Space Left On Device ---

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
#[allow(dead_code)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
    total_size: u32,
    processed: bool,
}

impl Default for Directory {
    fn default() -> Directory {
        Directory {
            name: String::new(),
            directories: Vec::new(),
            files: Vec::new(),
            total_size: 0,
            processed: false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    ChangeDirRoot,
    ChangeDir,
    ChangeDirUp,
    ListFiles,
    DirInfo,
    FileInfo,
    None,
}

#[derive(Debug)]
struct Command {
    action: Action,
    param: String,
}

impl Default for Command {
    fn default() -> Command {
        Command {
            action: Action::None,
            param: String::new(),
        }
    }
}

fn line_parser(line: &str) -> Command {
    let mut command = Command::default();

    if line.starts_with("$ cd ") {
        let param = line.strip_prefix("$ cd ").unwrap();
        if param == "/" {
            command.action = Action::ChangeDirRoot;
            command.param = param.to_string();
        } else if param == ".." {
            command.action = Action::ChangeDirUp;
        } else {
            command.action = Action::ChangeDir;
            command.param = param.to_string();
        }
    } else if line.starts_with("$ ls") {
        command = Command {
            action: Action::ListFiles,
            param: String::new(),
        }
    } else if line.starts_with("dir ") {
        let param = line.strip_prefix("dir ").unwrap();
        command = Command {
            action: Action::DirInfo,
            param: param.to_string(),
        }
    } else if line.chars().next().unwrap().is_numeric() {
        command = Command {
            action: Action::FileInfo,
            param: line.to_string(),
        }
    } else {
        panic!("Unknown command: {}", line);
    }

    command
}

fn get_dir<'a>(
    parent_dir: &'a mut Directory,
    dir_name: &String,
) -> Result<&'a mut Directory, String> {
    for dir in &mut (parent_dir.directories) {
        if dir.name == dir_name.clone() {
            return Ok(dir);
        }
    }

    return Err("Directory NOT found!".to_string());
}

fn get_cwd<'a>(dir_tree: &'a mut Directory, dir_stack: &Vec<String>) -> &'a mut Directory {
    let mut cwd = dir_tree;

    for i in 1..dir_stack.len() {
        let path = dir_stack[i].clone();
        cwd = get_dir(cwd, &path).unwrap();
    }

    cwd
}

fn has_dir(parent_dir: &mut Directory, dir_name: &String) -> bool {
    if get_dir(parent_dir, dir_name).is_ok() {
        return true;
    } else {
        return false;
    }
}

fn add_dir(parent_dir: &mut Directory, dir: Directory) {
    parent_dir.directories.push(dir);
}

fn add_file(parent_dir: &mut Directory, file: File) {
    parent_dir.files.push(file);
}

fn print_dir(dir: &Directory, level: i32) {
    let cwd = dir;
    let tab = 4;
    let spaces = (tab * level) as usize;

    for dir in &(cwd.directories) {
        print!("{}", " ".repeat(spaces));
        println!("- {} (dir, size={})", dir.name, dir.total_size);
        print_dir(dir, level + 1);
    }

    for file in &(cwd.files) {
        print!("{}", " ".repeat(spaces));
        println!("- {} (file, size={})", file.name, file.size);
    }
}

#[allow(dead_code)]
fn print_dir_tree(dir_tree: &Directory) {
    let cwd = dir_tree;

    println!("- {} (dir, size={})", cwd.name, cwd.total_size);
    print_dir(cwd, 1);
}

fn construct_dir_tree(file_content: String) -> Directory {
    let lines = file_content.lines();
    let mut dir_tree = Directory::default();
    let mut dir_stack: Vec<String> = Vec::new();

    for line in lines {
        let command = line_parser(line);

        match command.action {
            Action::ChangeDirRoot => {
                dir_stack.push(command.param.clone());
                dir_tree.name = command.param.clone();
            }
            Action::ChangeDir => {
                let cwd: &mut Directory = get_cwd(&mut dir_tree, &dir_stack);
                get_dir(cwd, &(command.param)).unwrap();
                dir_stack.push(command.param.clone());
            }
            Action::ChangeDirUp => {
                dir_stack.pop();
            }
            Action::ListFiles => {}
            Action::DirInfo => {
                let mut dir = Directory::default();
                dir.name = command.param.clone();
                let cwd: &mut Directory = get_cwd(&mut dir_tree, &dir_stack);

                if has_dir(cwd, &(dir.name)) == false {
                    add_dir(cwd, dir);
                }
            }
            Action::FileInfo => {
                let cwd: &mut Directory = get_cwd(&mut dir_tree, &dir_stack);
                let file_vec: Vec<String> = command
                    .param
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect();
                let file = File {
                    name: file_vec[1].clone(),
                    size: file_vec[0].parse::<u32>().unwrap(),
                };
                add_file(cwd, file);
            }
            Action::None => panic!("Empty command"),
        }
    }

    dir_tree
}

/// ALGO
/// Does the root dir was exit?
///     NO:
///         does this dir has other dirs with [processed = false] dir?
///             YES:
///                 enter that dir
///             NO:
///                 does this dir has other dirs?
///                     sum their sizes and add to this dir size
///                 does this dir has files?
///                     sum their sizes and add to this dir size
///                 exit this dir
///             REPEAT ALGO
///     YES:
///         EXIT ALGO
fn compute_dirs_total_sizes<'a>(dir_tree: &'a mut Directory) {
    let mut dir_stack: Vec<String> = vec!["/".to_string()];
    let mut cwd: &mut Directory = get_cwd(dir_tree, &dir_stack);

    while dir_stack.len() != 0 {
        let mut sub_dirs_processed = true;

        for dir in &(cwd.directories) {
            if dir.processed == false {
                dir_stack.push(dir.name.clone());
                cwd = get_cwd(dir_tree, &dir_stack);
                sub_dirs_processed = false;
                break;
            }
        }

        if sub_dirs_processed == true {
            for dir in &(cwd.directories) {
                cwd.total_size += dir.total_size;
            }

            for file in &(cwd.files) {
                cwd.total_size += file.size;
            }

            cwd.processed = true;
            dir_stack.pop();
            cwd = get_cwd(dir_tree, &dir_stack);
        }
    }
}

fn clear_processed_flags<'a>(dir_tree: &'a mut Directory) {
    let mut dir_stack: Vec<String> = vec!["/".to_string()];
    let mut cwd: &mut Directory = get_cwd(dir_tree, &dir_stack);

    while dir_stack.len() != 0 {
        let mut sub_dirs_processed = true;

        for dir in &(cwd.directories) {
            if dir.processed == true {
                dir_stack.push(dir.name.clone());
                cwd = get_cwd(dir_tree, &dir_stack);
                sub_dirs_processed = false;
                break;
            }
        }

        if sub_dirs_processed == true {
            cwd.processed = false;
            dir_stack.pop();
            cwd = get_cwd(dir_tree, &dir_stack);
        }
    }
}

fn sum_all_dirs_sizes<'a>(dir_tree: &'a mut Directory) -> u32 {
    let mut sum = 0;
    let mut dir_stack: Vec<String> = vec!["/".to_string()];
    let mut cwd: &mut Directory = get_cwd(dir_tree, &dir_stack);
    let size_limit = 100000;

    clear_processed_flags(cwd);

    while dir_stack.len() != 0 {
        let mut sub_dirs_processed = true;

        for dir in &(cwd.directories) {
            if dir.processed == false {
                dir_stack.push(dir.name.clone());
                cwd = get_cwd(dir_tree, &dir_stack);
                sub_dirs_processed = false;
                break;
            }
        }

        if sub_dirs_processed == true {
            for dir in &(cwd.directories) {
                if dir.total_size <= size_limit {
                    sum += dir.total_size;
                }
            }

            cwd.processed = true;
            dir_stack.pop();
            cwd = get_cwd(dir_tree, &dir_stack);
        }
    }

    sum
}

fn get_smallest_dir_size_with_limit<'a>(dir_tree: &'a mut Directory, size_limit: u32) -> u32 {
    let mut current_size = dir_tree.total_size;
    let mut dir_stack: Vec<String> = vec!["/".to_string()];
    let mut cwd: &mut Directory = get_cwd(dir_tree, &dir_stack);

    clear_processed_flags(cwd);

    while dir_stack.len() != 0 {
        let mut sub_dirs_processed = true;

        for dir in &(cwd.directories) {
            if dir.processed == false {
                dir_stack.push(dir.name.clone());
                cwd = get_cwd(dir_tree, &dir_stack);
                sub_dirs_processed = false;
                break;
            }
        }

        if sub_dirs_processed == true {
            for dir in &(cwd.directories) {
                if (dir.total_size >= size_limit) && (dir.total_size < current_size) {
                    current_size = dir.total_size;
                }
            }

            cwd.processed = true;
            dir_stack.pop();
            cwd = get_cwd(dir_tree, &dir_stack);
        }
    }

    current_size
}

fn solve_part_one(file_content: String) -> u32 {
    let mut dir_tree = construct_dir_tree(file_content);
    compute_dirs_total_sizes(&mut dir_tree);

    sum_all_dirs_sizes(&mut dir_tree)
}

fn solve_part_two(file_content: String) -> u32 {
    let total_disk_size = 70_000_000;
    let needed_space = 30_000_000;

    let mut dir_tree = construct_dir_tree(file_content);
    compute_dirs_total_sizes(&mut dir_tree);

    let space_to_be_emptied = dir_tree.total_size - (total_disk_size - needed_space);

    get_smallest_dir_size_with_limit(&mut dir_tree, space_to_be_emptied)
}

fn main() {
    let file_content = read_input_file();

    // --- Part One ---
    let result = solve_part_one(file_content.clone());
    println!(
        "Find all of the directories with a total size of at most 100000. \
        What is the sum of the total sizes of those directories? Answer: [{}]",
        result
    );

    // --- Part Two ---
    let result = solve_part_two(file_content.clone());

    println!(
        "Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. \
        What is the total size of that directory? Answer: [{}]",
        result
    );
}
