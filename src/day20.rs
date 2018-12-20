use crate::common::read_file_lines;
use std::collections::{HashMap as Map, HashSet as Set, VecDeque as Deque};

type Point = [i32; 2];
type Door = (Point, Point);

#[derive(Debug, Clone)]
enum MyRegex {
    Leaf(char),        // Single character
    Seq(Vec<MyRegex>), // Sequence of exprs
    Alt(Vec<MyRegex>), // Options for exprs
}

fn parse_myregex(iter: &mut Deque<char>) -> MyRegex {
    let mut options = vec![];
    let mut curr = vec![];

    while let Some(c) = iter.pop_front() {
        match c {
            '|' => {
                options.push(MyRegex::Seq(curr));
                curr = vec![];
            }
            '(' => {
                curr.push(parse_myregex(iter));
                assert!(iter.pop_front() == Some(')'));
            }
            'N' | 'S' | 'E' | 'W' => {
                curr.push(MyRegex::Leaf(c));
            }
            _ => {
                iter.push_front(c);
                break;
            }
        }
    }

    options.push(MyRegex::Seq(curr));
    MyRegex::Alt(options)
}

fn parse_input() -> MyRegex {
    let lines = read_file_lines("inputs/day20");
    let mut iter = lines[0].chars().collect::<Deque<_>>();

    assert!(iter.pop_front() == Some('^'));
    assert!(iter.pop_back() == Some('$'));

    parse_myregex(&mut iter)
}

fn walk_paths(node: &MyRegex, active: &Set<Point>, doors: &mut Set<Door>) -> Set<Point> {
    match node {
        MyRegex::Seq(vec) => vec
            .iter()
            .fold(active.clone(), |c, node| walk_paths(node, &c, doors)),
        MyRegex::Alt(vec) => vec
            .iter()
            .map(|node| walk_paths(node, active, doors))
            .flatten()
            .collect(),
        MyRegex::Leaf(c) => active
            .iter()
            .cloned()
            .map(|src| {
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

                dst
            })
            .collect(),
    }
}

fn find_room_dists(center: Point, doors: &Set<Door>) -> Map<Point, i32> {
    let mut adj = Map::<Point, Vec<Point>>::new();
    for (a, b) in doors {
        adj.entry(*a).or_insert(vec![]).push(*b);
    }

    let mut dists = Map::new();
    let mut queue = Deque::new();
    queue.push_back((center, 0));

    while let Some((v, d)) = queue.pop_front() {
        if !dists.contains_key(&v) {
            dists.insert(v, d);

            for u in &adj[&v] {
                queue.push_back((*u, d + 1));
            }
        }
    }

    dists
}

pub fn run(_: &[&str]) {
    let root = parse_input();

    let mut doors = Set::new();
    let mut active = Set::new();
    active.insert([0, 0]);

    let _ = walk_paths(&root, &active, &mut doors);
    let dists = find_room_dists([0, 0], &doors);

    let max_dist = dists.values().max();
    println!("answer A: {:?}", max_dist);

    let num_paths = dists.values().filter(|v| **v >= 1000).count();
    println!("answer B: {:?}", num_paths);
}
