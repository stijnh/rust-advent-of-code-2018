use crate::common::read_file_lines;
use ndarray::prelude::*;

type Cart = (char, i8, usize);

fn parse_input() -> (Array2<char>, Array2<Option<Cart>>) {
    let mut width = 0;
    let mut tracks = vec![];
    let mut carts = vec![];

    for (y, line) in read_file_lines("inputs/day13").iter().enumerate() {
        let mut line_width = 0;

        for c in line.chars() {
            line_width += 1;

            match c {
                '>' | '<' => {
                    carts.push(Some((c, 0, 0)));
                    tracks.push('-');
                }
                '^' | 'v' => {
                    carts.push(Some((c, 0, 0)));
                    tracks.push('|');
                }
                c => {
                    carts.push(None);
                    tracks.push(c);
                }
            }
        }

        if y == 0 {
            width = line_width;
        } else if line_width != width {
            panic!("uneven line widths one line {}", y);
        }
    }

    let shape = (tracks.len() / width, width);
    (
        Array2::from_shape_vec(shape, tracks).unwrap(),
        Array2::from_shape_vec(shape, carts).unwrap(),
    )
}

pub fn process_cell(
    tick: usize,
    i: usize,
    j: usize,
    tracks: &Array2<char>,
    carts: &mut Array2<Option<Cart>>,
) -> Option<(usize, usize)> {
    let (c, mut mem) = match carts[[i, j]] {
        Some((c, m, last_tick)) if last_tick < tick => {
            carts[[i, j]] = None;
            (c, m)
        }
        _ => return None,
    };

    let (ni, nj) = match c {
        '^' => (i - 1, j),
        '<' => (i, j - 1),
        '>' => (i, j + 1),
        'v' => (i + 1, j),
        _ => panic!("unknown cart symbol {}", c),
    };

    let nc = if tracks[[ni, nj]] == '+' {
        let old_mem = mem;
        mem = (mem + 1) % 3;

        match (c, old_mem) {
            ('^', 0) => '<',
            ('^', 1) => '^',
            ('^', 2) => '>',
            ('>', 0) => '^',
            ('>', 1) => '>',
            ('>', 2) => 'v',
            ('v', 0) => '>',
            ('v', 1) => 'v',
            ('v', 2) => '<',
            ('<', 0) => 'v',
            ('<', 1) => '<',
            ('<', 2) => '^',
            x => panic!("unknown pair {:?}", x),
        }
    } else {
        match (c, tracks[[ni, nj]]) {
            (_, '|') | (_, '-') => c,
            ('^', '/') => '>',
            ('>', '/') => '^',
            ('v', '/') => '<',
            ('<', '/') => 'v',
            ('^', '\\') => '<',
            ('>', '\\') => 'v',
            ('v', '\\') => '>',
            ('<', '\\') => '^',
            x => panic!("unknown pair {:?}", x),
        }
    };

    if let Some(_) = carts[[ni, nj]].take() {
        Some((ni, nj))
    } else {
        carts[[ni, nj]] = Some((nc, mem, tick));
        None
    }
}

pub fn run(_: &[&str]) {
    let (tracks, mut carts) = parse_input();
    let (width, height) = (tracks.shape()[0], tracks.shape()[1]);
    let mut num_carts = carts.iter().filter(|x| x.is_some()).count();
    let mut collisions = vec![];
    let mut tick = 0;

    while num_carts > 1 {
        tick += 1;
        println!("tick: {}, carts: {}", tick, num_carts);

        for i in 0..height {
            for j in 0..width {
                if let Some(p) = process_cell(tick, i, j, &tracks, &mut carts) {
                    collisions.push(p);
                    num_carts -= 2;
                }
            }
        }
    }

    let (y, x) = collisions[0];
    println!("answer A: {:?}", (x, y));

    let (y, x) = carts
        .indexed_iter()
        .filter(|(_, c)| c.is_some())
        .map(|(p, _)| p)
        .next()
        .unwrap();
    println!("answer B: {:?}", (x, y));
}
