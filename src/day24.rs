use crate::common::read_file_lines;
use enumset::EnumSet;
use regex::Regex;
use std::collections::HashSet;

#[derive(EnumSetType, Debug)]
enum Attack {
    Fire,
    Radiation,
    Slashing,
    Bludgeoning,
    Cold,
}

impl Attack {
    fn from(s: &str) -> Option<Self> {
        Some(match s {
            "fire" => Attack::Fire,
            "radiation" => Attack::Radiation,
            "slashing" => Attack::Slashing,
            "bludgeoning" => Attack::Bludgeoning,
            "cold" => Attack::Cold,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Team {
    Infection,
    Immune,
}

#[derive(Debug, Clone)]
struct Army {
    units: i64,
    hp: i64,
    damage: i64,
    initiative: i64,
    team: Team,
    immune: EnumSet<Attack>,
    weakness: EnumSet<Attack>,
    attack: Attack,
}

fn parse_input() -> Vec<Army> {
    let re = Regex::new(concat!(
        r"([0-9]+) units each with ([0-9]+) hit points ",
        r"(?:\(([a-z ,;]+)\) )?with an attack that does ",
        r"([0-9]+) ([a-z]+) damage at initiative ([0-9]+)",
    ))
    .unwrap();

    let lines = read_file_lines("inputs/day24");
    let mut team = Team::Infection;
    let mut armies = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        } else if &line == "Immune System:" {
            team = Team::Immune;
        } else if &line == "Infection:" {
            team = Team::Infection;
        } else if let Some(cap) = re.captures(&line) {
            let mut immune = EnumSet::new();
            let mut weakness = EnumSet::new();

            if let Some(c) = cap.get(3) {
                for part in c.as_str().split("; ") {
                    if part.starts_with("weak to ") {
                        for s in part[8..].split(", ") {
                            weakness |= Attack::from(s).unwrap();
                        }
                    } else if part.starts_with("immune to ") {
                        for s in part[10..].split(", ") {
                            immune |= Attack::from(s).unwrap();
                        }
                    } else {
                        panic!("unknown part: {:?}", part);
                    }
                }
            }

            let units = cap[1].parse().unwrap();
            let hp = cap[2].parse().unwrap();
            let damage = cap[4].parse().unwrap();
            let attack = Attack::from(&cap[5]).unwrap();
            let initiative = cap[6].parse().unwrap();

            armies.push(Army {
                immune,
                weakness,
                team,
                units,
                hp,
                damage,
                attack,
                initiative,
            });
        } else {
            panic!("failed to parse line: {:?}", line);
        }
    }

    armies
}

fn calc_damage(from: &Army, to: &Army) -> i64 {
    let mut dmg = from.units * from.damage;

    if from.team == to.team || to.units == 0 {
        dmg = 0;
    }

    if to.immune.contains(from.attack) {
        dmg = 0;
    }

    if to.weakness.contains(from.attack) {
        dmg *= 2;
    }

    dmg
}

fn target_round(armies: &[Army]) -> Vec<Option<usize>> {
    let n = armies.len();
    let mut targets = vec![None; n];

    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|i| (armies[*i].damage * armies[*i].units, armies[*i].initiative));
    indices.reverse();

    for i in indices {
        let mut target = None;
        let mut best_option = (0, 0, 0);

        for j in 0..n {
            if targets.contains(&Some(j)) {
                continue;
            }

            let dmg = calc_damage(&armies[i], &armies[j]);
            let pwr = armies[j].damage * armies[j].units;
            let initiative = armies[j].initiative;
            let option = (dmg, pwr, initiative);

            if option > best_option && dmg > 0 {
                target = Some(j);
                best_option = option;
            }
        }

        targets[i] = target;
    }

    targets
}

fn attack_round(armies: &mut [Army], targets: &[Option<usize>]) -> bool {
    let n = armies.len();
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|i| armies[*i].initiative);
    indices.reverse();

    let mut changes = false;

    for i in indices {
        if let Some(j) = targets[i] {
            let dmg = calc_damage(&armies[i], &armies[j]);
            let units = (dmg / armies[j].hp).min(armies[j].units);

            if units > 0 {
                armies[j].units -= units;
                changes = true
            }
        }
    }

    changes
}

fn simulate_war(armies: &mut [Army]) -> Option<Team> {
    while {
        let targets = target_round(armies);
        attack_round(armies, &targets)
    } {}

    let mut teams = armies
        .iter()
        .filter(|a| a.units > 0)
        .map(|a| a.team)
        .collect::<Vec<_>>();

    teams.sort();
    teams.dedup();

    if teams.len() == 1 {
        Some(teams[0])
    } else {
        None
    }
}

fn total_units(armies: &[Army]) -> i64 {
    armies.iter().map(|a| a.units).sum()
}

pub fn run(_: &[&str]) {
    let armies = parse_input();

    let mut simple = armies.clone();
    simulate_war(&mut simple);
    println!("answer A: {}", total_units(&simple));

    let mut result = vec![];

    for boost in 0.. {
        let mut armies = armies.clone();
        armies
            .iter_mut()
            .filter(|a| a.team == Team::Immune)
            .for_each(|a| a.damage += boost);

        let winner = simulate_war(&mut armies);
        println!("boost: {:?}, winner: {:?}", boost, winner);

        if winner == Some(Team::Immune) {
            result = armies;
            break;
        }
    }

    println!("answer B: {}", total_units(&result));
}
