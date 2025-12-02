use std::fs;

fn zero_pass_count(curr: i32, rot: &str) -> (i32, i32) {
    let mut count = 0;
    let (dir, num_) = rot.split_at(1);
    let mut num: i32 = num_.parse().unwrap();
    let sign = if dir == "L" { -1 } else { 1 };
    if num >= 100 {
        count += num.div_euclid(100);
        num %= 100;
    }
    let new_curr = curr + (sign * num);
    if curr == 0 {
    } else if new_curr == 0 {
        count += 1;
        println!("{}", rot)
    } else if (new_curr + 100) % 100 != new_curr {
        count += 1;
        println!("{}", rot)
    }

    ((new_curr + 100) % 100, count)
}

fn main() {
    let raw = fs::read_to_string("inputs/day1.txt").expect("Could not read file.");
    let lines: Vec<&str> = raw.lines().filter(|s| !s.trim().is_empty()).collect();

    let mut num: i32 = 50;

    let count: i32 = lines
        .iter()
        .map(|line| {
            let (n, c) = zero_pass_count(num, line);
            num = n;
            c
        })
        .sum();
    println!("{}", count);
}
