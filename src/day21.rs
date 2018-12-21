use crate::common::read_file_lines;
use regex::Regex;
use std::collections::HashSet;
use std::i64;

type Instr = (String, i64, i64, i64);
type Regs = [i64; 6];

fn parse_input() -> (usize, Vec<Instr>) {
    let instr_re = Regex::new(r"([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
    let ip_re = Regex::new(r"#ip (\d+)").unwrap();

    let lines = read_file_lines("inputs/day21");
    let mut ip = 0;
    let mut instrs = vec![];

    for line in lines {
        if let Some(cap) = ip_re.captures(&line) {
            ip = cap[1].parse().unwrap();
        } else if let Some(cap) = instr_re.captures(&line) {
            instrs.push((
                cap[1].to_string(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            ))
        } else {
            panic!("failed to match line {:?}", line);
        }
    }

    (ip, instrs)
}

fn exec_instr(instr: &Instr, mut regs: Regs) -> Regs {
    let to_int = |x| if x { 1 } else { 0 };

    let (name, a, b, c) = instr;
    let (a, b, c) = (*a, *b, *c);
    let (ua, ub, uc) = (a as usize, b as usize, c as usize);

    regs[uc] = match &**name {
        "addr" => i64::checked_add(regs[ua], regs[ub]).unwrap(),
        "addi" => i64::checked_add(regs[ua], b).unwrap(),
        "mulr" => i64::checked_mul(regs[ua], regs[ub]).unwrap(),
        "muli" => i64::checked_mul(regs[ua], b).unwrap(),
        "banr" => regs[ua] & regs[ub],
        "bani" => regs[ua] & b,
        "borr" => regs[ua] | regs[ub],
        "bori" => regs[ua] | b,
        "setr" => regs[ua],
        "seti" => a,
        "gtir" => to_int(a > regs[ub]),
        "gtri" => to_int(regs[ua] > b),
        "gtrr" => to_int(regs[ua] > regs[ub]),
        "eqir" => to_int(a == regs[ub]),
        "eqri" => to_int(regs[ua] == b),
        "eqrr" => to_int(regs[ua] == regs[ub]),
        _ => panic!("unknown instruction {:?}", name),
    };

    regs
}

pub fn run(_: &[&str]) {
    let (ip, instrs) = parse_input();
    let mut seen = HashSet::new();
    let mut values = vec![];

    let mut regs = [0; 6];
    loop {
        // the code from line 17 to 26 is just an ineffcient bitshift
        // skip this code an perform the bitshift directly.
        if regs[ip] == 17 {
            regs[4] = regs[1] >> 8;
            regs[ip] = 26;
        }

        // the code on line 28 checks if registers r0 and r3 matche.
        // store the value of r3 and break if we have seen the value before.
        if regs[ip] == 28 {
            let v = regs[3];

            if seen.insert(v) {
                values.push(v);
            } else {
                break;
            }
        }

        regs = exec_instr(&instrs[regs[ip] as usize], regs);
        regs[ip] += 1;
    }

    println!("answer A: {:?}", values.first());
    println!("answer B: {:?}", values.last());
}
