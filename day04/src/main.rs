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

            if i + 3 <= n - 1 {
                let mut maybe_xmas = String::new();
                maybe_xmas.push_str(&matrix[i][j]);
                maybe_xmas.push_str(&matrix[i+1][j]);
                maybe_xmas.push_str(&matrix[i+2][j]);
                maybe_xmas.push_str(&matrix[i+3][j]);
                if maybe_xmas == "XMAS" {
                    total += 1
                }
            }
            if j + 3 <= n - 1 {
                let mut maybe_xmas = String::new();
                maybe_xmas.push_str(&matrix[i][j]);
                maybe_xmas.push_str(&matrix[i][j+1]);
                maybe_xmas.push_str(&matrix[i][j+2]);
                maybe_xmas.push_str(&matrix[i][j+3]);
                if maybe_xmas == "XMAS" {
                    total += 1
                }
            }
            if ( i + 3 <= n - 1 ) && (j + 3 <= n - 1) {
                let mut maybe_xmas = String::new();
                maybe_xmas.push_str(&matrix[i][j]);
                maybe_xmas.push_str(&matrix[i+1][j+1]);
                maybe_xmas.push_str(&matrix[i+2][j+2]);
                maybe_xmas.push_str(&matrix[i+3][j+3]);
                if maybe_xmas == "XMAS" {
                    total += 1
                }
            }
        }
    }

    println!("{total}");
}
