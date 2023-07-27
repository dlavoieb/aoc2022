use std::cmp::{max, min};
use std::str::FromStr;

struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new_from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split("-").collect();
        Interval {
            start: i32::from_str(parts[0]).unwrap(),
            end: i32::from_str(parts[1]).unwrap(),
        }
    }
}

fn main() {
    let mut lines = aoc2022::read_file("src/bin/dec04/adventofcode.com_2022_day_4_input.txt");

    let mut contained = 0;
    let mut overlapping = 0;
    for line in lines {
        let intervals = parse_line(&line);
        if fully_contained(&intervals) {
            contained +=1;
        }
        if find_overlap(&intervals).is_some() {
            overlapping +=1;
        }
    }
    println!("{}", contained);
    println!("{}", overlapping);
}

fn parse_line(line: &String) -> (Interval, Interval) {
    let intervals: Vec<&str> = line.split(",").collect();
    (Interval::new_from_string(intervals[0]), Interval::new_from_string(intervals[1]))
}

fn fully_contained(pair: &(Interval, Interval)) -> bool {
    let (first, second) = pair;
    first.start <= second.start && first.end >= second.end ||
        second.start <= first.start && second.end >= first.end
}

fn find_overlap(pair: &(Interval, Interval)) -> Option<Interval> {
    let (first, second) = pair;
    if first.start <= second.start && first.end >= second.start{
        return Some(Interval{start: second.start, end: min(first.end, second.end)})
    }
    if second.start <= first.start && second.end >= first.start{
        return Some(Interval{start: first.start, end: min(second.end, first.end)})
    }

    None
}


#[cfg(test)]
mod tests {
    use crate::{find_overlap, fully_contained, Interval};

    #[test]
    fn contains() {
        assert!(fully_contained(&(
            Interval {
                start: 4,
                end: 7,
            },
            Interval {
                start: 3,
                end: 9,
            })));
        assert!(fully_contained(&(
            Interval {
                start: 2,
                end: 18,
            },
            Interval {
                start: 3,
                end: 9,
            })));
        assert!(fully_contained(&(
            Interval {
                start: 2,
                end: 9,
            },
            Interval {
                start: 3,
                end: 9,
            })));
    }

    #[test]
    fn overlap_1() {
        let overlap = find_overlap(&(
            Interval {
                start: 3,
                end: 7,
            },
            Interval {
                start: 3,
                end: 9,
            })).unwrap();
        assert_eq!(overlap.start, 3);
        assert_eq!(overlap.end, 7);
    }

    #[test]
    fn overlap_2() {
        let overlap = find_overlap(&(
            Interval {
                start: 4,
                end: 7,
            },
            Interval {
                start: 3,
                end: 7,
            })).unwrap();
        assert_eq!(overlap.start, 4);
        assert_eq!(overlap.end, 7);
    }

    #[test]
    fn overlap_3() {
        let overlap = find_overlap(&(
            Interval {
                start: 2,
                end: 7,
            },
            Interval {
                start: 3,
                end: 9,
            })).unwrap();
        assert_eq!(overlap.start, 3);
        assert_eq!(overlap.end, 7);
    }

    #[test]
    fn overlap_4() {
        let overlap = find_overlap(&(
            Interval {
                start: 6,
                end: 9,
            },
            Interval {
                start: 3,
                end: 7,
            })).unwrap();
        assert_eq!(overlap.start, 6);
        assert_eq!(overlap.end, 7);
    }

    #[test]
    fn overlap_5() {
        let overlap = find_overlap(&(
            Interval {
                start: 6,
                end: 9,
            },
            Interval {
                start: 3,
                end: 4,
            }));
        assert!(overlap.is_none());
    }
}