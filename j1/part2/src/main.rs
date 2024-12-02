use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (left, right) = read_file();
    let len = left.len();
    let mut count = 0;
    let hashmap = recurrence(left);
    for i in 0..len {
        let value = right[i];
        match hashmap.get(&value) {
            Some(recurrence) => {
                count += value * recurrence;
            }
            None => {}
        }
    }
    println!("Total : {}", count)
}

fn recurrence(list: Vec<i32>) -> HashMap<i32, i32> {
    let mut hashmap = HashMap::new();
    for i in list {
        if hashmap.contains_key(&i) {
            hashmap.insert(i, hashmap.get(&i).unwrap() + 1);
        } else {
            hashmap.insert(i, 1);
        }
    }
    return hashmap;
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
