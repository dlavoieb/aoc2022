use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Elf {
    food: Vec<i32>,
}

impl Elf {
    fn new() -> Self {
        Elf { food: Vec::new() }
    }

    fn push(&mut self, food_item: &str) {
        self.food.push(i32::from_str(food_item).unwrap());
    }

    fn sum(&self) -> i32 {
        self.food.iter().sum()
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.sum()).cmp(&other.sum())
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.sum() == other.sum()
    }
}

impl Eq for Elf {}

fn main() {
    let mut elves = read_file("src/bin/dec01/adventofcode.com_2022_day_1_input.txt");
    elves.sort();
    elves.reverse();

    println!("{}", elves[0].sum());
    println!("{}", total(&elves[0..2]));
}

fn read_file(filename: &str) -> Vec<Elf> {
    let mut elves = vec!();

    let lines = aoc2022::read_file(filename);

    let mut elf = Elf::new();

    for line in lines {
        if line.is_empty() {
            elves.push(elf);
            elf = Elf::new();
        } else {
            elf.push(&line);
        }
    }
    elves.push(elf);
    elves
}

fn total(elves: &[Elf]) -> i32 {
    let mut total = 0;
    for item in elves {
        total += item.sum();
    }
    total
}
