use binary_heap_plus::BinaryHeap;
use crate::common::read_file_lines;
use ndarray::prelude::*;
use regex::Regex;
use std::collections::hash_map::{Entry, HashMap};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    Gear,
    Empty,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Surface {
    Rocky,
    Wet,
    Narrow,
}

use self::Surface::*;
use self::Tool::*;

impl Surface {
    fn risk(&self) -> i32 {
        match self {
            Rocky => 0,
            Wet => 1,
            Narrow => 2,
        }
    }

    fn suitable(&self, tool: &Tool) -> bool {
        match (tool, self) {
            (Empty, Rocky) => false,
            (Torch, Wet) => false,
            (Gear, Narrow) => false,
            _ => true,
        }
    }
}

fn parse_input() -> (i32, [usize; 2]) {
    let re_depth = Regex::new("depth: ([0-9]+)").unwrap();
    let re_target = Regex::new("target: ([0-9]+),([0-9]+)").unwrap();
    let lines = read_file_lines("inputs/day22");

    let m = re_depth.captures(&lines[0]).unwrap();
    let depth = m[1].parse::<i32>().unwrap();

    let m = re_target.captures(&lines[1]).unwrap();
    let x = m[1].parse::<usize>().unwrap();
    let y = m[2].parse::<usize>().unwrap();

    (depth, [x, y])
}

fn build_cave(width: usize, height: usize, depth: i32, target: [usize; 2]) -> Array2<Surface> {
    let mut erosion = Array2::from_elem([width, height], 0);
    let mut cave = Array2::from_elem([width, height], Rocky);

    for x in 0..width {
        for y in 0..height {
            let index = match (x as i32, y as i32) {
                (0, 0) => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                (_, _) => erosion[[x - 1, y]] * erosion[[x, y - 1]],
            };

            erosion[[x, y]] = (index + depth) % 20183;
            cave[[x, y]] = match erosion[[x, y]] % 3 {
                0 => Rocky,
                1 => Wet,
                2 => Narrow,
                _ => unreachable!(),
            };
        }
    }

    cave[[0, 0]] = Rocky;
    cave[target] = Rocky;

    cave
}

fn find_paths(cave: &Array2<Surface>, target: [usize; 2]) -> i32 {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::new_by_key(|v: &(_, _, _, i32)| -v.3);

    let (width, height) = (cave.shape()[0] as isize, cave.shape()[1] as isize);
    let (tx, ty) = (target[0] as isize, target[1] as isize);

    queue.push((0, 0, Torch, 0));

    while let Some((x, y, tool, time)) = queue.pop() {
        match dists.entry((x, y, tool)) {
            Entry::Vacant(v) => v.insert(time),
            Entry::Occupied(_) => continue,
        };

        let adj = [
            (x - 1, y, tool, time + 1),
            (x + 1, y, tool, time + 1),
            (x, y - 1, tool, time + 1),
            (x, y + 1, tool, time + 1),
            (x, y, Torch, time + 7),
            (x, y, Gear, time + 7),
            (x, y, Empty, time + 7),
        ];

        for (x, y, tool, time) in adj.iter().cloned() {
            if x >= 0 && y >= 0 && x < width && y < height {
                if cave[[x as usize, y as usize]].suitable(&tool) {
                    queue.push((x, y, tool, time));
                }
            }
        }
    }

    if let Some(time) = dists.get(&(tx, ty, Torch)) {
        *time
    } else {
        panic!("path unreachable!");
    }
}

pub fn run(_: &[&str]) {
    let (depth, target) = parse_input();
    let (width, height) = (target[0] * 2, target[1] * 2);
    let cave = build_cave(width, height, depth, target);

    let total_risk = cave
        .slice(s![..target[0] + 1, ..target[1] + 1])
        .map(Surface::risk)
        .sum();
    println!("Answer A: {:?}", total_risk);

    let fastest = find_paths(&cave, target);
    println!("answer B: {:?}", fastest);
}
