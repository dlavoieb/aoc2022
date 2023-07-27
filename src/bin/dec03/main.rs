use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = aoc2022::read_file("src/bin/dec03/adventofcode.com_2022_day_3_input.txt");

    let mut score = 0;
    let mut new_score = 0;
    for line in &lines {
        let (left, right) = split_sack(&line);
        if let Some(letter) = find_duplicate(left, right) {
            score += priority(letter);
        }
    }
    println!("{}", score);

    for line_index in (0..lines.len()).step_by(3) {
        if let Some(letter) = find_common_item(&lines[line_index..line_index+3]) {
            new_score += priority(letter);
        }
    }
    println!("{}", new_score);

}

fn priority(letter: char) -> u32 {
    if letter.is_uppercase() {
        letter as u32 - 'A' as u32 + 27
    } else if letter.is_lowercase() {
        letter as u32 - 'a' as u32 + 1
    }
    else {
        0
    }
}

fn find_common_item(rucksacks: &[String]) -> Option<char> {
    for letter in rucksacks[0].chars() {
        if rucksacks[1].find(letter).is_some() {
            if rucksacks[2].find(letter).is_some() {
                return Some(letter);
            }
        }
    }
    None
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
