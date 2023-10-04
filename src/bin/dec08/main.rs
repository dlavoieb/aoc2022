use std::cmp::max;

fn main() {
    let lines = aoc2022::read_file("src/bin/dec08/adventofcode.com_2022_day_8_input.txt");
    let forest = build_forest(lines);
    let indexes = build_index_pairs(forest.len(), forest[0].len());

    let mut max_scenic_score = 0;
    let mut visibles = 0;
    for (row, column) in indexes {
        let current_tree_height = forest[row][column];

        let mut visible_from_above = true;
        let mut distance_above = 0;
        for row_above in (0..row).rev() {
            distance_above = row - row_above;
            if current_tree_height <= forest[row_above][column] {
                visible_from_above = false;
                break;
            }
        }

        let mut visible_from_below = true;
        let mut distance_below = 0;
        for row_below in row+1..forest.len() {
            distance_below = row_below - row;
            if current_tree_height <= forest[row_below][column] {
                visible_from_below = false;
                break;
            }
        }

        let mut visible_from_left = true;
        let mut distance_left = 0;
        for column_left in (0..column).rev() {
            distance_left = column - column_left;
            if current_tree_height <= forest[row][column_left] {
                visible_from_left = false;
                break;
            }
        }

        let mut visible_from_right = true;
        let mut distance_right = 0;
        for column_right in column+1..forest[0].len() {
            distance_right = column_right - column;
            if current_tree_height <= forest[row][column_right] {
                visible_from_right = false;
                break;
            }
        }

        if visible_from_above || visible_from_below || visible_from_right || visible_from_left {
            visibles += 1;
        }
        max_scenic_score = max(max_scenic_score, distance_above * distance_below * distance_left * distance_right);
    }
    println!("{}", visibles);
    println!("{}", max_scenic_score);
}

fn build_forest(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut forest = Vec::new();

    for line in lines {
        let mut row = Vec::new();

        for letter in line.chars() {
            row.push(letter.to_digit(10).unwrap() as i32);
        }
        forest.push(row);
    }
    forest
}

fn build_index_pairs(rows: usize, columns: usize) -> Vec<(usize, usize)> {
    let mut indexes = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            indexes.push((row, column));
        }
    }
    indexes
}