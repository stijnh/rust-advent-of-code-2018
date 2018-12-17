use crate::common::read_file_lines;
use ndarray::prelude::*;
use regex::Regex;

fn parse_input() -> Array2<char> {
    let re_xy = Regex::new(r"x=(\d+), y=(\d+)..(\d+)").unwrap();
    let re_yx = Regex::new(r"y=(\d+), x=(\d+)..(\d+)").unwrap();
    let mut points = vec![];

    for line in read_file_lines("inputs/day17") {
        let to_int = |s: &str| s.parse::<i32>().unwrap();

        let bounds = if let Some(cap) = re_xy.captures(&line) {
            (
                to_int(&cap[1]),
                to_int(&cap[1]),
                to_int(&cap[2]),
                to_int(&cap[3]),
            )
        } else if let Some(cap) = re_yx.captures(&line) {
            (
                to_int(&cap[2]),
                to_int(&cap[3]),
                to_int(&cap[1]),
                to_int(&cap[1]),
            )
        } else {
            panic!("failed to parse line {}", line);
        };

        points.push(bounds);
    }

    let y_lbnd = points.iter().map(|p| p.2).min().unwrap();
    let y_ubnd = points.iter().map(|p| p.3).max().unwrap();
    let height = (y_ubnd - y_lbnd + 1) as usize;
    let width = 1000;

    let mut ground = Array2::from_elem((width, height), ' ');

    for (x_min, x_max, y_min, y_max) in points {
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                ground[[x as usize, (y - y_lbnd) as usize]] = '#';
            }
        }
    }

    ground
}

fn fill_horizontal(
    sign: isize,
    [mut x, y]: [usize; 2],
    ground: &mut Array2<char>,
) -> (usize, bool) {
    loop {
        let nx = (x as isize + sign) as usize;

        match ground[[nx, y]] {
            ' ' => {
                ground[[nx, y]] = '~';

                if fill_down([nx, y], ground) {
                    break (nx, true);
                }
            }
            '~' => break (x, true),
            '#' => break (x, false),
            c => panic!("unexpected character {:?}", c),
        }

        x = nx;
    }
}

fn fill_right([x, y]: [usize; 2], ground: &mut Array2<char>) -> (usize, bool) {
    fill_horizontal(1, [x, y], ground)
}

fn fill_left([x, y]: [usize; 2], ground: &mut Array2<char>) -> (usize, bool) {
    fill_horizontal(-1, [x, y], ground)
}

fn fill_down([x, y]: [usize; 2], ground: &mut Array2<char>) -> bool {
    let mut dy = 1;
    let height = ground.shape()[1];

    loop {
        if y + dy >= height {
            return true;
        }

        match ground[[x, y + dy]] {
            ' ' => ground[[x, y + dy]] = '~',
            '#' | '-' => break,
            '~' => return true,
            c => panic!("unexpected character {:?}", c),
        }

        dy += 1;
    }

    loop {
        if dy <= 1 {
            break false;
        }

        dy -= 1;

        let (x_min, left_overflow) = fill_left([x, y + dy], ground);
        let (x_max, right_overflow) = fill_right([x, y + dy], ground);

        if left_overflow || right_overflow {
            break true;
        }

        for nx in x_min..=x_max {
            ground[[nx, y + dy]] = '-';
        }
    }
}

fn write_image(ground: &Array2<char>) {
    let width = ground.shape()[0] as u32;
    let height = ground.shape()[1] as u32;
    let mut img = image::RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = match ground[[x as usize, y as usize]] {
                ' ' => [0xff, 0xff, 0xff],
                '~' => [0x00, 0x00, 0xff],
                '-' => [0x80, 0x80, 0xff],
                '#' => [0x00, 0x00, 0x00],
                _ => [0xff, 0x00, 0x00],
            };

            img.put_pixel(x, y, image::Rgb(pixel));
        }
    }

    let filename = "day17.png";
    img.save(filename).unwrap();
    println!("saved image as {:?}", filename);
}

pub fn run(_: &[&str]) {
    let mut ground = parse_input();
    let spring: [usize; 2] = [500, 0];

    ground[spring] = '~';
    fill_down(spring, &mut ground);

    let answer_a = ground.iter().filter(|c| **c == '~' || **c == '-').count();

    let answer_b = ground.iter().filter(|c| **c == '-').count();

    println!("answer A: {}", answer_a);
    println!("answer B: {}", answer_b);

    write_image(&ground);
}
