use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let mut elves = vec!();

    if let Ok(file) = File::open("src/bin/dec01/adventofcode.com_2022_day_1_input.txt") {
        let reader = BufReader::new(file);

        let mut elf:Vec<i32> = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    elves.push(elf);
                    elf = Vec::new();
                } else {
                    elf.push(i32::from_str(&line).unwrap());
                }
            }
        }
        elves.push(elf);
    }

    let mut max = 0;
    for elf in elves {
        let sum = elf.iter().sum();
        if sum > max {
            max = sum;
        }
    }

    println!("{}", max);
}

