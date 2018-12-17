use crate::common::read_file_lines;

struct Node {
    children: Vec<Node>,
    entries: Vec<i32>,
}

fn parse_entry(data: &mut impl Iterator<Item = i32>) -> Node {
    let n = data.next().unwrap();
    let m = data.next().unwrap();
    let mut children = vec![];
    let mut entries = vec![];

    for _ in 0..n {
        children.push(parse_entry(data));
    }

    for _ in 0..m {
        entries.push(data.next().unwrap());
    }

    Node { children, entries }
}

fn sum_part_a(node: &Node) -> i32 {
    node.entries.iter().sum::<i32>() + node.children.iter().map(sum_part_a).sum::<i32>()
}

fn sum_part_b(node: &Node) -> i32 {
    if node.children.is_empty() {
        node.entries.iter().sum()
    } else {
        node.entries
            .iter()
            .map(|i| {
                if let Some(child) = node.children.get(*i as usize - 1) {
                    sum_part_b(&child)
                } else {
                    0
                }
            })
            .sum()
    }
}

pub fn run(_: &[&str]) {
    let line = read_file_lines("inputs/day8")[0].to_string();
    let mut iter = line.split(" ").map(|x| x.parse::<i32>().unwrap());

    let root = parse_entry(&mut iter);

    println!("answer A: {}", sum_part_a(&root));
    println!("answer B: {}", sum_part_b(&root));
}
