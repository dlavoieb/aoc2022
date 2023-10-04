use std::str::FromStr;

fn main() {
    let lines = aoc2022::read_file("src/bin/dec10/adventofcode.com_2022_day_10_input.txt");

    let mut signal = vec!(1);

    for line in lines {
        if line.starts_with("noop") {
            signal.push(signal.last().unwrap().clone());
        }
        else if line.starts_with("addx") {
            signal.push(signal.last().unwrap().clone());
            signal.push(signal.last().unwrap() + i32::from_str(&line[5..line.len()]).unwrap());
        }
    }

    let mut total = 0;
    let indexes = [20, 60, 100, 140, 180, 220];
    for index in indexes {
        total = total + index * signal[(index-1) as usize];
    }
    println!("{}", total);

    let mut render = Vec::new();
    for pixel_index in 0..signal.len()-1 {
        if pixel_index % 40 >= (signal[pixel_index] - 1) as usize && pixel_index % 40 <= (signal[pixel_index] + 1) as usize {
            render.push("#");
        }
        else {
            render.push(".");
        }
    }

    for row in 0..render.len() / 40 {
        for col in 0..40 {
            print!("{}", render[row * 40 + col]);
        }
        print!("\n");
    }

}