use crate::common::read_file_lines;
use std::iter::Peekable;
use std::collections::{HashSet, HashMap};
use std::mem::swap;
use ndarray::prelude::*;

type Point = [i32; 2];
type Door = (Point, Point);

#[derive(Debug)]
enum Node {
    Leaf(char),
    Seq(Vec<Node>),
    Alt(Vec<Node>),
}


fn parse_node(iter: &mut Peekable<impl Iterator<Item = char>>) -> Node {
    let mut options = vec![];
    let mut curr = vec![];

    while let Some(c) = iter.peek() {
        match c {
            '|' => {
                assert!(iter.next() == Some('|'));
                options.push(Node::Seq(curr));
                curr = vec![];
            },
            '(' => {
                assert!(iter.next() == Some('('));
                let v = parse_node(iter);
                assert!(iter.next() == Some(')'));
                curr.push(v);
            },
            'N' | 'S' | 'E' | 'W' => {
                let c = iter.next().unwrap();
                curr.push(Node::Leaf(c));
            },
            _ => break,
        }
    }

    options.push(Node::Seq(curr));
    Node::Alt(options)
}

fn parse_input() -> Node {
    let lines = read_file_lines("inputs/day20");
    let mut iter = lines[0].chars().peekable();

    assert!(iter.next() == Some('^'));
    let root = parse_node(&mut iter);
    assert!(iter.next() == Some('$'));
    assert!(iter.next() == None);

    root
}

fn simulate(node: &Node, active: &HashSet<Point>, doors: &mut HashSet<Door>) -> HashSet<Point> {
    match node {
        Node::Seq(vec) => {
            let mut current = active.clone();

            for node in vec {
                current = simulate(node, &current, doors);
            }

            current
        },
        Node::Alt(vec) => {
            let mut end = HashSet::new();

            for node in vec {
                end.extend(simulate(node, active,  doors));
            }

            end
        },
        Node::Leaf(c) => {
            let mut new_active = HashSet::new();

            for src in active.iter().cloned() {
                let [x, y] = src;
                let dst = match c {
                    'N' => [x, y + 1],
                    'S' => [x, y - 1],
                    'E' => [x + 1, y],
                    'W' => [x - 1, y],
                    _ => panic!("unknown symbol {:?}", c),
                };

                doors.insert((src, dst));
                doors.insert((dst, src));

                new_active.insert(dst);
            }

            new_active
        }
    }
}

fn find_room_dists(center: Point, doors: &HashSet<Door>) -> HashMap<Point, i32> {
    let mut adj = HashMap::<Point, Vec<Point>>::new();
    for (a, b) in doors {
        adj.entry(*a).or_insert(vec![]).push(*b);
    }

    let mut round = 0;
    let mut dists = HashMap::new();
    let mut frontier = vec![center];
    let mut next_frontier = vec![];

    while !frontier.is_empty() {
        for v in &frontier {
            if !dists.contains_key(v) {
                dists.insert(*v, round);

                for u in &adj[v] {
                    next_frontier.push(*u);
                }
            }
        }

        frontier = next_frontier;
        next_frontier = vec![];
        round += 1;
    }

    dists
}

pub fn run(_: &[&str]) {
    let root = parse_input();

    let mut doors = HashSet::new();
    let mut active = HashSet::new();

    active.insert([0, 0]);

    let _ = simulate(&root, &active, &mut doors);
    let dists = find_room_dists([0, 0], &doors);

    let max_dist = dists.values().max();
    println!("answer A: {:?}", max_dist);

    let num_paths = dists.values().filter(|v| **v >= 1000).count();
    println!("answer B: {:?}", num_paths);
}
