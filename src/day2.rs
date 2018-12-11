use crate::common::read_file_lines;
use std::collections::HashMap;

pub fn chars_difference(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

pub fn run(_: &[&str]) {
    let codes = read_file_lines("inputs/day2");
    let mut counts = HashMap::<char, i32>::new();
    let mut two_count = 0;
    let mut three_count = 0;

    for code in &codes {
        counts.clear();

        for c in code.chars() {
            *counts.entry(c).or_default() += 1;
        }

        if counts.values().any(|x| *x == 2) {
            two_count += 1;
        }

        if counts.values().any(|x| *x == 3) {
            three_count += 1;
        }
    }

    println!("answer A: {}", two_count * three_count);

    for a in &codes {
        for b in &codes {
            let diff = chars_difference(a, b);

            if diff == 1 {
                println!("answer B: {} {}", a, b);
            }
        }
    }
}
