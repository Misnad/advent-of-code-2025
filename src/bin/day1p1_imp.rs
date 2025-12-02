use std::fs;

fn parse_rotation(rot: &str) -> i32 {
    let sign = if rot.starts_with("L") { -1 } else { 1 };
    sign * rot[1..]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Cannot parse {}", rot))
}

fn main() {
    let raw = fs::read_to_string("inputs/day1.txt").expect("Could not read file.");
    let lines: Vec<&str> = raw.lines().filter(|s| !s.trim().is_empty()).collect();

    let mut num: i32 = 50;

    let count = lines
        .iter()
        .map(|line| {
            num += parse_rotation(line);
            num % 100
        })
        .filter(|&n| n == 0)
        .count();
    println!("{}", count);
}
