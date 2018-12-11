use std::collections::HashSet;
use crate::common::read_file_lines;

pub fn run(_: &[&str]) {
    let lines = read_file_lines("inputs/day1")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut value = 0i64;
    for delta in lines.iter() {
        value += delta;
    }

    println!("answer A: {}", value);

    let mut seen = HashSet::new();
    value = 0;

    for delta in lines.iter().cycle() {
        value += delta;

        if !seen.insert(value) {
            break;
        }
    }
    
    println!("answer B: {}", value);
}
