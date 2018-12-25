use crate::common::read_file_lines;
use itertools::enumerate;
use std::collections::HashSet;

pub fn run(_: &[&str]) {
    let mut points: Vec<[i64; 4]> = vec![];
    let mut labels = vec![];
    let lines = read_file_lines("inputs/day25");

    for line in lines {
        let mut parts = line.split(",");
        points.push([
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ]);

        labels.push(points.len());
    }

    for (i, p) in enumerate(&points) {
        for (j, q) in enumerate(&points) {
            let dist = (p[0] - q[0]).abs()
                + (p[1] - q[1]).abs()
                + (p[2] - q[2]).abs()
                + (p[3] - q[3]).abs();

            if dist <= 3 && labels[i] != labels[j] {
                for l in labels.iter_mut() {
                    if *l == labels[i] {
                        *l = labels[j];
                    }
                }
            }
        }
    }

    let uniq_labels = labels.iter().collect::<HashSet<_>>();
    println!("answer A: {}", uniq_labels.len());
}
