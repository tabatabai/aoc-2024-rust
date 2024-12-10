use std::fs;

fn is_safe(numbers: &Vec<i32>) -> bool {
    let mut increasing: bool = false;
    let mut decreasing: bool = false;
    for i in 0..numbers.len() - 1 {
        let diff = (numbers[i] - numbers[i + 1]).abs();
        if (diff < 1) | (diff > 3) {
            return false;
        } else if numbers[i] > numbers[i + 1] {
            decreasing = true;
        } else if numbers[i] < numbers[i + 1] {
            increasing = true;
        }
        if decreasing && increasing {
            return false;
        }
    }
    true
}

fn is_safe_with_removal(numbers: &Vec<i32>) -> bool {
    for i in 0..numbers.len() {
        let mut candidate = numbers.clone();
        candidate.remove(i);
        if is_safe(&candidate) {
            return true;
        }
    }
    false
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("input exists");
    let line_iterator = input.lines();
    let mut num_safe: u32 = 0;
    let mut num_safe_with_removal: u32 = 0;
    for line in line_iterator {
        let mut numbers_in_line: Vec<i32> = Vec::new();
        for number in line.split_whitespace() {
            numbers_in_line.push(number.parse().expect("parsable"));
        }
        if is_safe(&numbers_in_line) {
            num_safe += 1;
            num_safe_with_removal += 1;
        } else if is_safe_with_removal(&numbers_in_line) {
            num_safe_with_removal += 1;
        }
    }
    println!("Result part 1: {num_safe}");
    println!("Result part 2: {num_safe_with_removal}");
}
