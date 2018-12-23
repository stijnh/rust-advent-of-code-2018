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

    // initial volume is entire space
    let mut vols = vec![(0, 0, [-step, -step, -step])];
    step *= 2;

    while step >= 1 {
        let mut new_vols = vec![];

        // generate new volumes of each octant
        for (_, _, [cx, cy, cz]) in vols {
            for (i, j, k) in iproduct!(0..2, 0..2, 0..2) {
                let ax = cx + i * step;
                let ay = cy + j * step;
                let az = cz + k * step;

                let bx = ax + step - 1;
                let by = ay + step - 1;
                let bz = az + step - 1;

                // compute 2*distance to center
                let dist = (ax + bx).abs() + (ay + by).abs() + (az + bz).abs();

                // compute upper bound for bots in range of volume
                let mut count = 0;
                for bot in &bots {
                    let mut cost = 0;
                    cost += (ax - bot[0]).max(0);
                    cost += (bot[0] - bx).max(0);
                    cost += (ay - bot[1]).max(0);
                    cost += (bot[1] - by).max(0);
                    cost += (az - bot[2]).max(0);
                    cost += (bot[2] - bz).max(0);

                    if cost <= bot[3] {
                        count += 1;
                    }
                }

                new_vols.push((count, dist, [ax, ay, az]));
            }
        }

        // keep the best 10,000 volumes
        new_vols.sort_by(|a, b| iff!(a.0 != b.0, (a.0).cmp(&b.0).reverse(), (a.1).cmp(&b.1)));
        new_vols.truncate(10000);
        println!("best for step {}: {:?}", step, new_vols[0]);

        vols = new_vols;
        step /= 2;
    }

    let (count, _, [x, y, z]) = vols[0];
    println!(
        "answer B: {} ({} nanobots in range)",
        x.abs() + y.abs() + z.abs(),
        count
    );
}
