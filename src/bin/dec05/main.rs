use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let lines = aoc2022::read_file("src/bin/dec05/adventofcode.com_2022_day_5_input.txt");

    let container_row_index = find_container_row(&lines);
    let mut stacks = parse_container_setup(&lines, container_row_index);

    process_commands(&lines, container_row_index, &mut stacks);

    let mut text = String::new();
    for stack in stacks {
        text.push(stack.last().unwrap().clone());
    }

    println!("{}", text);
}

fn process_commands(lines: &Vec<String>, container_row_index: usize, stacks: &mut Vec<Vec<char>>) {
    for line_index in container_row_index + 2..lines.len() {
        let (num_containers, from, to) = parse_command(&lines, line_index);
        for _ in 0..num_containers {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val);
        }
    }
}

fn parse_command(lines: &Vec<String>, line_index: usize) -> (i32, usize, usize) {
    let first_split: Vec<&str> = lines[line_index][5..].split("from").collect();
    let num_containers = i32::from_str(&*first_split[0].replace(" ", "")).unwrap();
    let second_split: Vec<&str> = first_split[1].split("to").collect();
    let from = usize::from_str(&*second_split[0].replace(" ", "")).unwrap()-1;
    let to = usize::from_str(&*second_split[1].replace(" ", "")).unwrap()-1;
    (num_containers, from, to)
}

fn parse_container_setup(lines: &Vec<String>, container_row_index: usize) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let mut stack_index = Vec::new();

    for index in 0..lines[container_row_index].len() {
        if lines[container_row_index].chars().nth(index).unwrap().is_ascii_digit() {
            stack_index.push(index);
            stacks.push(Vec::new());
        }
    }

    for line_index in (0..container_row_index).rev() {
        for container_index in 0..stacks.len() {
            let char_index = stack_index[container_index];
            if let Some(container) = lines[line_index].chars().nth(char_index) {
                if container != ' ' {
                    stacks[container_index].push(container);
                }
            }
        }
    }
    stacks
}

fn find_container_row(lines: &Vec<String>) -> usize {
    let mut index = 0;

    for line in lines.iter() {
        if line.is_empty() {
            index -= 1;
            break;
        }
        index += 1;
    }
    index
}