use crate::common::read_file_lines;
use regex::Regex;
use std::collections::HashSet;

pub fn run(_: &[&str]) {
    const SIZE: usize = 1000;
    let mut fabric = vec![vec![(0, -1); SIZE]; SIZE];
    let mut nonoverlapping = HashSet::new();

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in read_file_lines("inputs/day3") {
        let cap = re.captures(&line).unwrap();

        let numbers = cap
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let claim = numbers[0] as i32;
        let x0 = numbers[1];
        let y0 = numbers[2];
        let x1 = x0 + numbers[3];
        let y1 = y0 + numbers[4];
        let mut is_nonoverlapping = true;

        for x in x0..x1 {
            for y in y0..y1 {
                let part = &mut fabric[x][y];
                if part.0 > 0 {
                    nonoverlapping.remove(&part.1);
                    is_nonoverlapping = false;
                }

                part.0 += 1;
                part.1 = claim;
            }
        }

        if is_nonoverlapping {
            nonoverlapping.insert(claim);
        }
    }

    let total = fabric
        .iter()
        .map(|v| v.iter())
        .flatten()
        .filter(|x| x.0 > 1)
        .count();

    println!("answer A: {}", total);
    println!("answer B: {:?}", nonoverlapping);
}
