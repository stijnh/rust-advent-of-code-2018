use crate::common::read_file_lines;
use regex::Regex;

type Instr = (String, i32, i32, i32);
type Regs = [i32; 6];

fn parse_input() -> (usize, Vec<Instr>) {
    let instr_re = Regex::new(r"([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
    let ip_re = Regex::new(r"#ip (\d+)").unwrap();

    let lines = read_file_lines("inputs/day19");
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
            println!("failed to match line {:?}", line);
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
        "addr" => regs[ua] + regs[ub],
        "addi" => regs[ua] + b,
        "mulr" => regs[ua] * regs[ub],
        "muli" => regs[ua] * b,
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

fn run_program(ip: usize, instrs: &[Instr], mut regs: Regs) -> usize {
    loop {
        // intercept at instruction 1
        if regs[ip] == 1 {
            break;
        }

        regs = exec_instr(&instrs[regs[ip] as usize], regs);
        regs[ip] += 1;
    }

    // the answer is the sum of factors of register 2.
    let n = regs[2] as usize;
    (1..=n).filter(|x| n % x == 0).sum()
}

pub fn run(_: &[&str]) {
    let (ip, instrs) = parse_input();

    let mut regs = [0; 6];
    println!("answer A: {:?}", run_program(ip, &instrs, regs));

    regs[0] = 1;
    println!("answer B: {:?}", run_program(ip, &instrs, regs));
}
