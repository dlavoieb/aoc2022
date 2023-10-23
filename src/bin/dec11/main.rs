use std::str::FromStr;

fn main() {
    let lines = aoc2022::read_file("src/bin/dec11/input.txt");

    let mut monkeys = build_starting_condition(lines);

    let mut testing_number = 1;
    for monkey in &monkeys {
        testing_number *= monkey.test_operand;
    }

    for _round in 0..10000 {
        for monkey in 0..monkeys.len() {
            let results = monkeys[monkey].process_all_items();
            for (des, value) in results {
                monkeys[des as usize].items.push(value % testing_number); // modulo to keep things manageable
            }
        }
    }

    let mut total_moves = Vec::new();
    for monkey in &monkeys {
        total_moves.push(monkey.total_inspections);
    }
    total_moves.sort();
    println!("{:?}", total_moves[total_moves.len() - 1] * total_moves[total_moves.len() - 2]);


    println!("Got {} monkeys", monkeys.len());
}

fn build_starting_condition(lines: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey = -1;

    for line in lines {
        if line.starts_with("Monkey") {
            let parts: Vec<&str> = line.strip_suffix(':').unwrap().split(" ").collect();
            current_monkey = i64::from_str(parts[1]).unwrap();
            monkeys.push(Monkey { operation: addition, operation_operand: -1, test_operand: -1, items: Vec::new(), monkey_true: -1, monkey_false: -1, total_inspections: 0 });
        } else if line.contains("Starting items") {
            let parts: Vec<&str> = line.split(": ").collect();
            let parts: Vec<&str> = parts[1].split(", ").collect();
            for part in parts {
                monkeys[current_monkey as usize].items.push(i64::from_str(part).unwrap());
            }
        } else if line.contains("Operation") {
            if line.contains("+") {
                let parts: Vec<&str> = line.split("+ ").collect();
                monkeys[current_monkey as usize].operation = addition;
                monkeys[current_monkey as usize].operation_operand = i64::from_str(parts[1]).unwrap();
            } else if line.contains("*") {
                let parts: Vec<&str> = line.split("* ").collect();
                if parts[1] == "old" {
                    monkeys[current_monkey as usize].operation = square;
                } else {
                    monkeys[current_monkey as usize].operation = multiplication;
                    monkeys[current_monkey as usize].operation_operand = i64::from_str(parts[1]).unwrap();
                }
            }
        } else if line.contains("Test") {
            let parts: Vec<&str> = line.split("by ").collect();
            monkeys[current_monkey as usize].test_operand = i64::from_str(parts[1]).unwrap();
        } else if line.contains("true") {
            let parts: Vec<&str> = line.split("monkey ").collect();
            monkeys[current_monkey as usize].monkey_true = i64::from_str(parts[1]).unwrap();
        } else if line.contains("false") {
            let parts: Vec<&str> = line.split("monkey ").collect();
            monkeys[current_monkey as usize].monkey_false = i64::from_str(parts[1]).unwrap();
        }
    }
    monkeys
}

struct Monkey {
    operation: fn(i64, i64) -> i64,
    operation_operand: i64,
    test_operand: i64,
    items: Vec<i64>,
    monkey_true: i64,
    monkey_false: i64,
    total_inspections: i64,
}

impl Monkey {
    fn process_item(& mut self) -> (i64, i64) {
        let old = self.items.pop().unwrap();
        let new = (self.operation)(old, self.operation_operand);
        // let new = new / 3;
        self.total_inspections += 1;
        if new % self.test_operand == 0 {
            (self.monkey_true, new)
        }
        else {
            (self.monkey_false, new)
        }
    }

    fn process_all_items(& mut self) -> Vec<(i64, i64)> {
        let mut results = Vec::new();
        for _item in 0..self.items.len() {
            results.push(self.process_item());
        }
        results
    }
}

fn addition(a: i64, b: i64) -> i64 {
    a + b
}

fn multiplication(a: i64, b: i64) -> i64 {
    a * b
}

fn square(a: i64, _b: i64) -> i64 {
    a * a
}