use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let mut lines = aoc2022::read_file("src/bin/dec06/adventofcode.com_2022_day_6_input.txt");
    let signal = lines.pop().unwrap();

    find_start(&signal, 4);
    find_start(&signal, 14);
}

fn find_start(signal: &String, length: usize) {
    for index in 0..signal.len() - length {
        let segment = &signal[index..index + length];
        if has_unique_elements(segment.chars()) {
            print!("{}", index + length);
            break;
        }
    }
}

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}