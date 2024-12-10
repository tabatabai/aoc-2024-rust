use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("input exists");
    let line_iter = input.lines();
    let mut matrix: Vec<Vec<String>> = Vec::new();
    for line in line_iter{
        let mut row: Vec<String> = Vec::new();
        for char in line.chars() {
            row.push(char.to_string());
        }
        matrix.push(row);
    }

    let mut total: u32 = 0;
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] == "X" {
                total += 1;
            }
        }
    }

    println!("{total}");
}
