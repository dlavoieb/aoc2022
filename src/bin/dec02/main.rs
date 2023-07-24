use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Ok(file) = File::open("src/bin/dec02/adventofcode.com_2022_day_2_input.txt") {
        let reader = BufReader::new(file);

        let mut score = 0;

        for line in reader.lines() {
            if let Ok(line) = line {
                score += round_score(&line);
            }
        }
        println!("{}", score);
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn outcome(game: &str) -> Outcome {
    match game {
        "A X" => { Outcome::Draw }
        "A Y" => { Outcome::Win }
        "A Z" => { Outcome::Loss }
        "B X" => { Outcome::Loss }
        "B Y" => { Outcome::Draw }
        "B Z" => { Outcome::Win }
        "C X" => { Outcome::Win }
        "C Y" => { Outcome::Loss }
        "C Z" => { Outcome::Draw }
        _ => { Outcome::Loss }
    }
}

fn item_score(round: &str) -> i32 {
    match round.chars().last().unwrap() {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn round_score(round: &str) -> i32 {
    let outcome = outcome(round);

    let score = item_score(round);
    match outcome {
        Outcome::Win => {score + 6}
        Outcome::Loss => {score}
        Outcome::Draw => {score + 3}
    }
}

#[cfg(test)]
mod tests {
    use crate::{item_score, outcome};
    use crate::Outcome::Win;

    #[test]
    fn test_outcomes() {
        assert_eq!(outcome("A Y"), Win);
    }

    #[test]
    fn test_score() {
        assert_eq!(item_score("A Y"), 2);
    }
}