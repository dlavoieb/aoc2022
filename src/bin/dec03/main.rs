use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = read_file("src/bin/dec03/adventofcode.com_2022_day_3_input.txt");

    let mut score = 0;
    for line in lines {
        let (left, right) = split_sack(&line);
        if let Some(letter) = find_duplicate(left, right) {
            if letter.is_uppercase() {
                score += letter as u32 - 'A' as u32 + 27;
            }
            else if letter.is_lowercase() {
                score += letter as u32 - 'a' as u32 + 1;
            }
        }
    }
    println!("{}", score);

}

fn read_file(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    }
    lines
}

fn split_sack(rucksack : &String) -> (&str, &str) {
    let midpoint = rucksack.len()/2;
    let left = &rucksack[0..midpoint];
    let right = &rucksack[midpoint..rucksack.len()];
    (left, right)
}

fn find_duplicate(left: &str, right: &str) -> Option<char> {
    for letter in left.chars() {
        if right.find(letter).is_some() {
            return Some(letter);
        }
    }
    None
}
