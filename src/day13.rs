use crate::common::read_file_lines;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

use self::Dir::*;

pub fn run(_: &[&str]) {
    let mut tracks = vec![];
    let mut carts: Vec<(i32, i32, Dir, i8)> = vec![];

    for (y, line) in read_file_lines("inputs/day13").iter().enumerate() {
        let mut track = vec![];

        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => {
                    carts.push((x as i32, y as i32, East, 0));
                    track.push('-');
                },
                '<' => {
                    carts.push((x as i32, y as i32, West, 0));
                    track.push('-');
                },
                '^' => {
                    carts.push((x as i32, y as i32, North, 0));
                    track.push('|');
                },
                'v' => {
                    carts.push((x as i32, y as i32, South, 0));
                    track.push('|');
                },
                c => track.push(c)
            };
        }

        tracks.push(track);
    }

    let mut occupied: HashSet<(i32, i32)> = carts
        .iter()
        .map(|(x, y, _, _)| (*x, *y))
        .collect::<HashSet<_>>();


    let collisions = vec![];

    while carts.len() > 1 {
        carts.sort_by_key(|(x, y, _, _)| (*y, *x));

        let mut index = 0;
        let mut old_carts = carts;
        carts = vec![];

        while index < old_carts.len() {
            let (x, y, dir, mut mem) = old_carts[index;
            let (dx, dy) = match dir {
                North => (0, -1),
                South => (0, 1),
                East => (1, 0),
                West => (-1, 0)
            };

            let (nx, ny) = (x + dx, y + dy);
            println!("{:?} '{}' => {:?} '{}'", 
                     (x, y), 
                     tracks[y as usize][x as usize],
                     (nx, ny),
                     tracks[ny as usize][nx as usize]);

            if occupied.contains(&(nx, ny)) {
                occupied.remove(&(nx, ny));
                occupied.remove(&(x, y));
                collision.push((nx, ny));

                

                continue;
            }

            let new_dir = match tracks[ny as usize][nx as usize] {
                '/' => match dir {
                    North => East,
                    East => North,
                    South => West,
                    West => South,
                },

                '\\' => match dir {
                    North => West,
                    West => North,
                    East => South,
                    South => East,
                },

                '+' => {
                    let old_mem = mem;
                    mem = (mem + 1) % 3;

                    match old_mem {
                        0 => match dir {
                            North => West,
                            West => South,
                            South => East,
                            East => North,
                        },
                        2 => match dir {
                            North => East,
                            East => South,
                            South => West,
                            West => North,
                        },
                        _ => dir,
                    }
                },

                '-' | '|' => dir,

                c => panic!("unknown track {} at {:?}", c, (nx, ny)),

            };

            occupied.remove(&(x, y));
            occupied.insert((nx, ny));
            carts.push((nx, ny, new_dir, mem));

            index += 1;
        }
    };

    println!("answer A: {:?}", collision);
}
