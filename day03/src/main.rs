use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("input exists");
    // let re = Regex::new(r"^mul(\d+,\d+)$").unwrap();
    let re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();
    let captures = re.captures_iter(&input);
    let mut total: i32 = 0;
    for d in captures {
        // TODO: the numbers themselves can be extracted by capture groups instead of doing a second match
        println!("{}", &d[0]);
        let mut new_term: i32 = 1;
        let number_captures = number_re.captures_iter(&d[0]);
        for x in number_captures {
            new_term *= &x[0].parse().expect("parsable");
        }
        total += new_term;
    }
    println!("Day 3, part 1: {total}")
}
