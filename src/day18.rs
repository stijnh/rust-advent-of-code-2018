use crate::common::read_file_lines;
use ndarray::prelude::*;

fn parse_input() -> Array2<char> {
    let lines = read_file_lines("inputs/day18");
    let size = lines.len();

    let mut cells = Array2::from_elem((size + 2, size + 2), '.');
    let mut inner = cells.slice_mut(s![1..-1, 1..-1]);

    for (row, line) in izip!(inner.genrows_mut(), lines) {
        for (entry, c) in izip!(row, line.chars()) {
            *entry = c;
        }
    }

    cells
}

#[inline(never)]
fn simulate(current: &Array2<char>, next: &mut Array2<char>) {
    let inner = next.slice_mut(s![1..-1, 1..-1]);
    let windows = current.windows([3, 3]);

    for (entry, win) in izip!(inner, windows) {
        let mut num_yards = 0;
        let mut num_trees = 0;

        for c in win {
            num_yards += iff!(*c == '#', 1, 0);
            num_trees += iff!(*c == '|', 1, 0);
        }

        *entry = match win[[1, 1]] {
            '.' => iff!(num_trees >= 3, '|', '.'),
            '|' => iff!(num_yards >= 3, '#', '|'),
            '#' => iff!(num_trees >= 1 && num_yards >= 2, '#', '.'),
            x => panic!("unknown symbol {:?}", x),
        };
    }
}

pub fn run(_: &[&str]) {
    let mut current = parse_input();
    let mut next = current.clone();
    let mut scores = vec![];

    // just simulate for 1000 timesteps
    for _ in 0..1000 {
        let mut num_yards = 0;
        let mut num_trees = 0;
        for c in current.iter() {
            match c {
                '#' => num_yards += 1,
                '|' => num_trees += 1,
                _ => {}
            }
        }

        scores.push((num_yards, num_trees));

        simulate(&current, &mut next);
        std::mem::swap(&mut current, &mut next);
    }

    // part A: score after 10 minutes
    println!("answer A: {}", scores[10].0 * scores[10].1);

    // part B: score after 1000000000 minutes. We assume there is some
    // kind of cyclic pattern of length "cycle" and then we can go
    // back in time some amount of k * cycle steps.
    let n = scores.len();
    let mut cycle = 1;

    loop {
        let a = &scores[n - cycle..];
        let b = &scores[n - 2 * cycle..n - cycle];

        if a != b {
            cycle += 1;
        } else {
            break;
        }
    }

    let mut i = 1000000000;
    while i >= n {
        i -= cycle;
    }

    println!("answer B: {}", scores[i].0 * scores[i].1);
}
