use crate::common::read_file_lines;
use itertools::enumerate;
use ndarray::prelude::*;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct Entity {
    pos: [usize; 2],
    attack: i32,
    health: i32,
    is_elf: bool,
}

fn parse_input() -> (Vec<Entity>, Array2<bool>) {
    let lines = read_file_lines("inputs/day15");

    let width = lines[0].len();
    let height = lines.len();

    let mut walls = Array2::from_elem((width, height), false);
    let mut entities = vec![];

    for (y, line) in enumerate(lines) {
        assert_eq!(width, line.len());

        for (x, c) in enumerate(line.chars()) {
            if c == 'G' || c == 'E' {
                entities.push(Entity {
                    pos: [x, y],
                    attack: 3,
                    health: 200,
                    is_elf: c == 'E',
                });
            } else if c == '#' || c == '.' {
                walls[[x, y]] = c == '#';
            } else {
                panic!("unknown symbol: {:?}", c);
            }
        }
    }

    (entities, walls)
}

fn flood(obstacle: &Array2<bool>, start: [usize; 2]) -> Array2<Option<i32>> {
    let (width, height) = obstacle.dim();
    let mut result = Array2::from_elem((width, height), None);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some(([x, y], d)) = queue.pop_front() {
        if obstacle[[x, y]] || result[[x, y]].is_some() {
            continue;
        }

        result[[x, y]] = Some(d);

        if x > 0 {
            queue.push_back(([x - 1, y], d + 1));
        }

        if y > 0 {
            queue.push_back(([x, y - 1], d + 1));
        }

        if x + 1 < width {
            queue.push_back(([x + 1, y], d + 1));
        }

        if y + 1 < height {
            queue.push_back(([x, y + 1], d + 1));
        }
    }

    result
}

// calculate new position of entity i if it would move
fn calc_entity_move(i: usize, entities: &mut [Entity], obstacle: &Array2<bool>) -> [usize; 2] {
    let e = &entities[i];
    let [x, y] = e.pos;
    let dists = flood(&obstacle, [x, y]);
    let mut options = vec![];

    for enemy in entities.iter() {
        if enemy.is_elf == e.is_elf || enemy.health == 0 {
            continue;
        }

        let [ex, ey] = enemy.pos;
        let deltas = [[ex - 1, ey], [ex + 1, ey], [ex, ey - 1], [ex, ey + 1]];

        for [px, py] in deltas.iter().cloned() {
            if let Some(d) = dists[[px, py]] {
                options.push((d, [px, py]));
            }
        }
    }

    let (_, [tx, ty]) = options
        .into_iter()
        .min_by_key(|(d, [x, y])| (*d, *y, *x))
        .unwrap_or((0, e.pos));

    if [tx, ty] != e.pos {
        let dists = flood(&obstacle, [tx, ty]);
        let up = dists[[x, y - 1]].unwrap_or(10000);
        let down = dists[[x, y + 1]].unwrap_or(10000);
        let left = dists[[x - 1, y]].unwrap_or(10000);
        let right = dists[[x + 1, y]].unwrap_or(10000);
        let min = up.min(down).min(left).min(right);

        if up == min {
            [x, y - 1]
        } else if left == min {
            [x - 1, y]
        } else if right == min {
            [x + 1, y]
        } else {
            [x, y + 1]
        }
    } else {
        e.pos
    }
}

// calculate which entity would be attacked by entity i
fn calc_entity_attack(i: usize, entities: &mut [Entity]) -> Option<usize> {
    let e = &entities[i];
    let [x, y] = e.pos;
    let mut options = vec![];

    for (j, enemy) in enumerate(entities.iter()) {
        if enemy.health == 0 || enemy.is_elf == e.is_elf {
            continue;
        }

        let up = enemy.pos == [x, y - 1];
        let down = enemy.pos == [x, y + 1];
        let left = enemy.pos == [x - 1, y];
        let right = enemy.pos == [x + 1, y];

        if up || down || left || right {
            options.push((j, enemy));
        }
    }

    let best = options
        .into_iter()
        .min_by_key(|(_, x)| (x.health, x.pos[1], x.pos[0]));

    match best {
        Some((j, _)) => Some(j),
        None => None,
    }
}

fn print_arena(entities: &[Entity], walls: &Array2<bool>) {
    let (width, height) = walls.dim();

    for y in 0..height {
        let mut row = vec![];

        for x in 0..width {
            let mut c = ' ';

            if walls[[x, y]] {
                c = '#';
            }

            for e in entities.iter() {
                if e.pos == [x, y] && e.health > 0 {
                    c = iff!(e.is_elf, 'E', 'G');
                    row.push(e);
                }
            }

            print!("{}", c);
        }

        print!(" ");

        for e in row {
            print!("{} ", e.health);
        }

        println!();
    }
}

fn simulate_war(entities: &mut [Entity], walls: &Array2<bool>) -> i32 {
    let mut obstacle = walls.clone();
    for entity in entities.iter() {
        obstacle[entity.pos] = true;
    }

    let mut rounds = 0;

    loop {
        let mut indices = (0..entities.len()).collect::<Vec<_>>();
        indices.sort_by_key(|i| {
            let [x, y] = entities[*i].pos;
            (y, x)
        });

        for i in indices {
            if entities[i].health == 0 {
                continue;
            }

            obstacle[entities[i].pos] = false;

            entities[i].pos = calc_entity_move(i, entities, &obstacle);
            if let Some(j) = calc_entity_attack(i, entities) {
                let dmg = entities[i].attack;
                let e = &mut entities[j];

                if e.health > dmg {
                    e.health -= dmg;
                } else {
                    e.health = 0;
                    obstacle[e.pos] = false;
                }
            }

            obstacle[entities[i].pos] = true;
        }

        let teams = entities
            .iter()
            .filter(|e| e.health > 0)
            .fold(0, |v, e| v | iff!(e.is_elf, 1, 2));

        if teams < 3 {
            break rounds;
        }

        //print_arena(&entities, &walls);
        rounds += 1;
    }
}

pub fn run(_: &[&str]) {
    let (entities, walls) = parse_input();

    {
        let mut entities = entities.clone();
        let rounds = simulate_war(&mut entities, &walls);

        let total_health = entities.iter().map(|e| e.health).sum::<i32>();
        println!("answer A: {}", total_health * rounds);
    }

    for attack in 4.. {
        println!("trying attack: {}", attack);

        let mut entities = entities.clone();
        for e in &mut entities {
            if e.is_elf {
                e.attack = attack;
            }
        }

        let rounds = simulate_war(&mut entities, &walls);
        let elves_dead = entities.iter().filter(|e| e.is_elf).any(|e| e.health == 0);

        if !elves_dead {
            let total_health = entities.iter().map(|e| e.health).sum::<i32>();
            println!("answer B: {}", total_health * rounds);
            break;
        }
    }
}
