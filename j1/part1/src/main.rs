use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num;
fn main() {
    let (mut left, mut right) = read_file();
    let len = left.len();
    left.sort();
    right.sort();
    let mut count = 0;
    for i in 0..len {
        let diff: i32 = left[i] - right[i];
        count += diff.abs();
    }
    println!("Total : {}", count)
}

fn read_file() -> (Vec<i32>, Vec<i32>) {
    let mut right: Vec<i32> = Vec::new();
    let mut left: Vec<i32> = Vec::new();
    let file = File::open("file/input.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let res = parse_line(line);
        if res.is_none() {
            continue;
        }
        let (left_value, right_value) = res.unwrap();
        left.push(left_value);
        right.push(right_value);
    }
    return (left, right);
}

fn parse_line(line: String) -> Option<(i32, i32)> {
    let right: i32;
    let left: i32;
    if line.len() == 0 {
        return None;
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    right = parts[1].parse::<i32>().unwrap();
    left = parts[0].parse::<i32>().unwrap();
    return Some((left, right));
}
