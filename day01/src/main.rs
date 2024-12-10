use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("input exists");
    let lines = input.lines();
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        let left = numbers.next().unwrap();
        let right = numbers.next().unwrap();
        let left: i32 = left.parse().unwrap();
        let right: i32 = right.parse().unwrap();
        left_numbers.push(left);
        right_numbers.push(right);
    }
    left_numbers.sort();
    right_numbers.sort();

    let mut total: i32 = 0;
    let n = left_numbers.len();
    for i in 0..n {
        total += (left_numbers[i] - right_numbers[i]).abs();
    }
    println!("Result: {total}\n");

    let mut total_2: i32 = 0;
    for l in &left_numbers {
        let mut count_in_right: i32 = 0;
        for r in &right_numbers {
            if l == r {
                count_in_right += 1;
            }
        }
        total_2 += count_in_right * l;
    }
    println!("Result 2: {total_2}\n");
}
