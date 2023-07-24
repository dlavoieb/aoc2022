use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Ok(file) = File::open("src/bin/dec02/adventofcode.com_2022_day_2_input.txt") {
        let reader = BufReader::new(file);

        let mut score = 0;
        let mut new_score = 0;
        for line in reader.lines() {
            if let Ok(line) = line {
                score += round_score(&line);
                let opponent = opponent_move(&line);
                let desired_outcome = desired_outcome(&line);
                let my_move = move_to_play(&desired_outcome, opponent);
                new_score += item_score_move(my_move);
                new_score += outcome_score(&desired_outcome);


            }
        }
        println!("{}", score);
        println!("{}", new_score);
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

fn item_score_move(round: Move) -> i32 {
    match round {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3
    }
}

fn round_score(round: &str) -> i32 {
    let outcome = outcome(round);

    let score = item_score(round);
    score + outcome_score(&outcome)
}

fn outcome_score(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Win => {6}
        Outcome::Loss => {0}
        Outcome::Draw => {3}
    }
}

enum Move {
    Rock,
    Paper,
    Scissor
}

fn move_to_play(desired_outcome: &Outcome, opponent: Move) -> Move {
    match desired_outcome {
        Outcome::Win => {
            match opponent {
                Move::Rock => {Move::Paper}
                Move::Paper => {Move::Scissor}
                Move::Scissor => {Move::Rock}
            }
        }
        Outcome::Loss => {
            match opponent {
                Move::Rock => {Move::Scissor}
                Move::Paper => {Move::Rock}
                Move::Scissor => {Move::Paper}
            }
        }
        Outcome::Draw => { opponent }
    }
}

fn desired_outcome(round: &str) -> Outcome {
    match round.chars().last().unwrap() {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn opponent_move(round: &str) -> Move {
    match round.chars().next().unwrap() {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissor,
        _ => Move::Rock,
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