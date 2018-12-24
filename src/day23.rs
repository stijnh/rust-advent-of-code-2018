use binary_heap_plus::BinaryHeap;
use crate::common::read_file_lines;
use regex::Regex;

type Bot = [i64; 4];

fn parse_input() -> Vec<Bot> {
    let re = Regex::new(r"pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, r=([0-9]+)").unwrap();
    let lines = read_file_lines("inputs/day23");
    let mut rows = vec![];

    for line in lines {
        let cap = re.captures(&line).unwrap();
        let row = [
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
        ];

        rows.push(row);
    }

    rows
}

pub fn run(_: &[&str]) {
    let bots = parse_input();

    let largest = bots.iter().max_by_key(|bot| bot[3]).unwrap().clone();
    let count = bots
        .iter()
        .filter(move |bot| {
            let mut dist = 0;
            dist += (largest[0] - bot[0]).abs();
            dist += (largest[1] - bot[1]).abs();
            dist += (largest[2] - bot[2]).abs();
            dist <= largest[3]
        })
        .count();

    println!("answer A: {}", count);

    let mut step = 1;
    loop {
        let out = bots
            .iter()
            .filter(|bot| {
                let mut d = 0;
                d = bot[0].abs().max(d);
                d = bot[1].abs().max(d);
                d = bot[2].abs().max(d);
                d > step
            })
            .count();

        if out > 0 {
            step *= 2;
        } else {
            break;
        }
    }

    let mut best = (0, [0, 0, 0]);

    // initial volume is entire space
    let mut queue = BinaryHeap::new_by_key(|v: &(i64, i64, i64, _)| (v.0, v.2, -v.1));
    queue.push((0, 0, 2 * step, [-step, -step, -step]));

    while let Some((c, _, step, [cx, cy, cz])) = queue.pop() {
        if step == 1 {
            best = (c, [cx, cy, cz]);
            break;
        }

        // generate new volumes of each octant
        for (i, j, k) in iproduct!(0..2, 0..2, 0..2) {
            let half_step = step / 2;

            let ax = cx + i * half_step;
            let ay = cy + j * half_step;
            let az = cz + k * half_step;

            let bx = ax + half_step - 1;
            let by = ay + half_step - 1;
            let bz = az + half_step - 1;

            // compute 2*distance to center
            let dist = (ax + bx).abs() + (ay + by).abs() + (az + bz).abs();

            // compute upper bound for bots in range of volume
            let mut max_count = 0;

            for bot in &bots {
                let mut min_cost = 0;
                min_cost += (ax - bot[0]).max(0) + (bot[0] - bx).max(0);
                min_cost += (ay - bot[1]).max(0) + (bot[1] - by).max(0);
                min_cost += (az - bot[2]).max(0) + (bot[2] - bz).max(0);

                if min_cost <= bot[3] {
                    max_count += 1;
                }
            }

            queue.push((max_count, dist, half_step, [ax, ay, az]));
        }
    }

    let (count, [x, y, z]) = best;
    println!(
        "answer B: {} ({} nanobots in range)",
        x.abs() + y.abs() + z.abs(),
        count
    );
}
