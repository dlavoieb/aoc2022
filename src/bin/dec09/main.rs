use std::str::FromStr;

fn main() {
    let lines = aoc2022::read_file("src/bin/dec09/adventofcode.com_2022_day_9_input.txt");

    let mut visited_positions: Vec<(i32, i32)> = Vec::new();
    visited_positions.push((0, 0));

    let mut short_rope = create_rope(2);
    let mut long_rope = create_rope(10);

    for line in lines {
        let commands :Vec<&str> = line.split(" ").collect();
        let direction = commands[0];
        let times = i32::from_str(commands[1]).unwrap();

        for _ in 0..times {
            short_rope[0] = move_head(short_rope[0], direction);
            short_rope[1] = new_tail_position(short_rope[1], &short_rope[0]);

            long_rope[0] = move_head(long_rope[0], direction);
            for index in 0..long_rope.len()-1 {
                long_rope[index + 1] = new_tail_position(long_rope[index + 1], &long_rope[index]);
            }

            add_tail_if_new(&mut visited_positions, long_rope.last().unwrap());
        }
    }

    println!("{}", visited_positions.len());
}

fn add_tail_if_new(visited_positions: &mut Vec<(i32, i32)>, tail: &(i32, i32)) {
    let mut visited = false;
    for visited_position in visited_positions.into_iter() {
        if tail == visited_position {
            visited = true;
            break;
        }
    }
    if !visited {
        visited_positions.push(tail.clone());
    }
}

fn move_head(head: (i32, i32), direction: &str) -> (i32, i32){
    let (mut current_head_x, mut current_head_y) = head;
    match direction {
        "U" => { current_head_y = current_head_y + 1; }
        "D" => { current_head_y = current_head_y - 1; }
        "R" => { current_head_x = current_head_x + 1; }
        "L" => { current_head_x = current_head_x - 1; }
        _ => {}
    }
    (current_head_x, current_head_y)
}

fn new_tail_position(past_tail_position: (i32, i32), current_head_position: &(i32, i32)) -> (i32, i32) {
    return if head_and_tail_touching(current_head_position, &past_tail_position) {
        past_tail_position
    } else {
        let (head_x, head_y) = current_head_position;
        let (tail_x, tail_y) = past_tail_position;
        if head_x == &tail_x {
            // movement along the y axis, which direction?
            let movement = tail_y - head_y;
            let sign = if movement == movement.abs() { -1 }  else { 1 };
            (tail_x, tail_y + sign)
        } else if head_y == &tail_y {
            // movement along the x axis, which direction?
            let movement = tail_x - head_x;
            let sign = if movement == movement.abs() { -1 }  else { 1 };
            (tail_x + sign, tail_y)
        } else {
            // diagonal movement, which direction?
            let movement_x = tail_x - head_x;
            let movement_y = tail_y - head_y;
            let sign_x = if movement_x == movement_x.abs() { -1 }  else { 1 };
            let sign_y = if movement_y == movement_y.abs() { -1 }  else { 1 };
            (tail_x + sign_x, tail_y + sign_y)
        }

    }
}

fn head_and_tail_touching(head_position: &(i32, i32), tail_position: &(i32, i32)) -> bool {
    let (head_x, head_y) = head_position;
    let (tail_x, tail_y) = tail_position;

    return (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1;
}

fn create_rope(length: i32) -> Vec<(i32, i32)> {
    let mut rope = Vec::new();
    for _ in 0..length {
        rope.push((0,0));
    }
    rope
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touching_test() {
        let head = (0, 0);
        let tail = (0, 0);
        assert!(head_and_tail_touching(&head, &tail));

        let head = (0, 0);
        let tail = (0, 1);
        assert!(head_and_tail_touching(&head, &tail));

        let head = (1, 0);
        let tail = (0, 1);
        assert!(head_and_tail_touching(&head, &tail));

        let head = (0, 0);
        let tail = (0, 2);
        assert!(!head_and_tail_touching(&head, &tail));
    }

    #[test]
    fn movement_test() {
        let head = (0, 0);
        let tail = (0, 0);
        let new_tail = new_tail_position(tail, &head);
        assert_eq!(new_tail, (0,0));

        let head = (0, 0);
        let tail = (0, 2);
        let new_tail = new_tail_position(tail, &head);
        assert_eq!(new_tail, (0,1));

        let head = (2, 0);
        let tail = (0, 0);
        let new_tail = new_tail_position(tail, &head);
        assert_eq!(new_tail, (1,0));

        let head = (2, 1);
        let tail = (0, 0);
        let new_tail = new_tail_position(tail, &head);
        assert_eq!(new_tail, (1,1));
    }
}