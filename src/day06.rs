use crate::common::read_file_lines;

pub fn area_per_center(
    centers: &[(i32, i32)],
    (xmin, xmax): (i32, i32),
    (ymin, ymax): (i32, i32),
) -> Vec<i32> {
    let mut area = vec![0; centers.len()];

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            let mut best = (-1, 0);

            for (index, coord) in centers.iter().enumerate() {
                let dist = (x - coord.0).abs() + (y - coord.1).abs();

                if dist < best.1 || index == 0 {
                    best = (index as i32, dist);
                } else if dist == best.1 {
                    best = (-1, dist);
                }
            }

            if best.0 != -1 {
                area[best.0 as usize] += 1;
            }
        }
    }

    area
}

pub fn total_distance_less_than_1000(
    centers: &[(i32, i32)],
    (xmin, xmax): (i32, i32),
    (ymin, ymax): (i32, i32),
) -> i32 {
    let mut output = 0;

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            let total_dist: i32 = centers
                .iter()
                .map(|(cx, cy)| (cx - x).abs() + (cy - y).abs())
                .sum();

            if total_dist < 10000 {
                output += 1;
            }
        }
    }

    output
}

pub fn run(_: &[&str]) {
    let coords = read_file_lines("inputs/day6")
        .into_iter()
        .map(|line| {
            let index = line.find(',').unwrap();
            (line[..index].to_string(), line[index + 2..].to_string())
        })
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    let a = area_per_center(&coords, (-1500, 2500), (-1500, 2500));
    let b = area_per_center(&coords, (-2500, 3500), (-2500, 3500));

    let result = a.iter().zip(b.iter()).filter(|v| v.0 == v.1).max().unwrap();

    println!("answer A: {:?}", result);

    let result = total_distance_less_than_1000(&coords, (-2500, 3500), (-2500, 3500));
    println!("answer B: {:?}", result);
}
