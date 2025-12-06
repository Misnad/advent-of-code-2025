use std::fs;

fn is_invalid(code: i64) -> bool {
    let c = code.to_string();
    let n = c.len();
    c[..n / 2] == c[n / 2..]
}

fn main() {
    let raw = fs::read_to_string("inputs/day2.txt").unwrap();
    let inp: Vec<&str> = raw.trim().split(",").collect();

    let nums: Vec<i64> = inp
        .iter()
        .flat_map(|v| {
            let c: Vec<&str> = v.split("-").collect();
            c[0].parse::<i64>().unwrap_or_else(|v| panic!("{v}"))..c[1].parse::<i64>().unwrap() + 1
        })
        .collect();

    let mut invalid: Vec<i64> = Vec::new();
    for i in nums {
        if is_invalid(i) {
            invalid.push(i);
        }
    }
    println!("{}", invalid.iter().sum::<i64>());
}
