use crate::common::read_file_lines;
use std::collections::HashMap;
use std::convert::TryInto;

pub fn run(_: &[&str]) {
    let lines = read_file_lines("inputs/day12");

    // Load initial state
    let header = "initial state: ";
    let mut current = lines[0][header.len()..].chars().collect::<Vec<_>>();

    // Load rules
    let mut rules = HashMap::new();
    for line in &lines[2..] {
        let chars = line.chars().collect::<Vec<_>>();

        let v: &[char; 5] = (&chars[..5]).try_into().unwrap();
        let a = chars[9];

        rules.insert(*v, a);
    }

    let mut first_plant = 0;
    let mut scores = vec![];

    // Simulate for 2500 generations
    for _ in 0..2500usize {
        let offset = current.iter().position(|x| *x == '#').unwrap_or(0);
        let roffset = current.iter().rposition(|x| *x == '#').unwrap_or(0) + 1;

        let mut next = vec![];
        next.extend(vec!['.'; 4]);
        next.extend_from_slice(&current[offset..roffset]);
        next.extend(vec!['.'; 4]);
        first_plant += offset as i32 - 4;

        current = next.windows(5).map(|m| rules[m]).collect::<Vec<_>>();
        first_plant += 2;

        let total = current
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == '#')
            .map(|(i, _)| first_plant as i64 + i as i64)
            .sum::<i64>();

        scores.push(total);
    }

    // Question A: score at generation 20
    println!("answer A: {}", scores[19]);

    // Question B: score at generation 50000000000
    // Assuming growth is linear, we just need to find increment per gen
    let n = scores.len();
    let incr = scores[n - 1] - scores[n - 2];
    let cons = scores[n - 1] - incr * (n as i64 - 1);

    println!("answer B: {}", cons + (50000000000i64 - 1) * incr);
}
