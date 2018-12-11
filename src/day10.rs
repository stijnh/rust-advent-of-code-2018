use crate::common::read_file_lines;
use regex::Regex;

pub fn run(_: &[&str]) {
    let mut points = vec![];
    let re =
        Regex::new(r"position=<([-0-9 ]+),([-0-9 ]+)> velocity=<([-0-9 ]+),([-0-9 ]+)>").unwrap();

    for line in read_file_lines("inputs/day10") {
        let matches = re.captures(&line).unwrap();
        let vec = matches
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str())
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        points.push((vec[0], vec[1], vec[2], vec[3]));
    }

    // Find time where bounds on stars is minimal
    let mut best_time = 0;
    let mut best_area = 10000;
    let mut best_bounds = (0, 0, 0, 0);

    for time in 0..20000 {
        let mut bounds = None;

        for (cx, cy, dx, dy) in &points {
            let x = cx + time * dx;
            let y = cy + time * dy;

            bounds = match bounds {
                Some((ax, ay, bx, by)) => {
                    let ax = i64::min(ax, x);
                    let ay = i64::min(ay, y);
                    let bx = i64::max(bx, x);
                    let by = i64::max(by, y);
                    Some((ax, ay, bx, by))
                }
                None => Some((x, y, x, y)),
            };
        }

        let bounds = bounds.unwrap();
        let area = (bounds.2 - bounds.0) * (bounds.3 - bounds.1);

        if area < best_area {
            best_time = time;
            best_area = area;
            best_bounds = bounds;
        }
    }

    // Draw stars
    let (min_x, min_y, max_x, max_y) = best_bounds;
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut screen = vec![vec!['.'; width as usize]; height as usize];

    for (cx, cy, dx, dy) in &points {
        let x = cx + best_time * dx;
        let y = cy + best_time * dy;

        screen[(y - min_y) as usize][(x - min_x) as usize] = '#';
    }

    println!("time: {}", best_time);

    for line in screen {
        println!("{}", line.iter().collect::<String>());
    }
}
