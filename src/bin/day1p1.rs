use std::fs;

fn parse_rotation(rot: &str) -> i32 {
    let mut num: i32 = rot[1..].parse().expect("Cannot parse");
    match rot.chars().nth(0).unwrap() {
        'L' => num *= -1,
        _ => {}
    }
    num
}

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").expect("Could not read file.");
    let mut input: Vec<&str> = input.split("\n").collect();
    input.retain(|s| !s.trim().is_empty());
    // println!("{:?}", input);

    let mut num: i32 = 50;
    let mut count: i64 = 0;
    for i in input {
        num += parse_rotation(i);
        num %= 100;
        if num == 0 {
            count += 1
        }
    }
    println!("{}", count);
}
