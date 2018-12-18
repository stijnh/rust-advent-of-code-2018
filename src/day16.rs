use crate::common::read_file_lines;
use regex::Regex;
use std::default::Default;

type Instr = [i32; 4];
type Regs = [i32; 4];

fn parse_captures(re: &Regex, line: &str) -> [i32; 4] {
    if let Some(cap) = re.captures(line) {
        let mut output = [0; 4];

        cap.iter()
            .skip(1)
            .map(|x| x.unwrap().as_str())
            .map(|x| x.parse::<i32>().expect(x))
            .enumerate()
            .for_each(|(i, v)| output[i] = v);

        output
    } else {
        panic!("failed regex '{}' on '{}'", re.as_str(), line);
    }
}

const NUM_OPCODES: usize = 16;

fn exec_instr(opcode: i32, a: i32, b: i32, c: i32, mut regs: Regs) -> Regs {
    let to_int = |b| if b { 1 } else { 0 };

    let ra = regs[a as usize];
    let rb = regs[b as usize];
    let rc = match opcode {
        0 => ra + rb,
        1 => ra + b,
        2 => ra * rb,
        3 => ra * b,
        4 => ra & rb,
        5 => ra & b,
        6 => ra | rb,
        7 => ra | b,
        8 => ra,
        9 => a,
        10 => to_int(a > rb),
        11 => to_int(ra > b),
        12 => to_int(ra > rb),
        13 => to_int(a == rb),
        14 => to_int(ra == b),
        15 => to_int(ra == rb),
        _ => panic!("unknown opcode {}", opcode),
    };

    regs[c as usize] = rc;
    regs
}

fn name_instr(opcode: i32) -> &'static str {
    let names = [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];

    names.get(opcode as usize).unwrap_or(&"unknown")
}

fn parse_input() -> (Vec<(Regs, Instr, Regs)>, Vec<Instr>) {
    let before_re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let instr_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let after_re = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    let mut index = 0;
    let lines = read_file_lines("inputs/day16");
    let mut samples = vec![];
    let mut program = vec![];

    while !lines[index].is_empty() {
        let before = parse_captures(&before_re, &lines[index]);
        let instr = parse_captures(&instr_re, &lines[index + 1]);
        let after = parse_captures(&after_re, &lines[index + 2]);
        index += 4;

        samples.push((before, instr, after));
    }

    index += 2;

    while index < lines.len() {
        let instr = parse_captures(&instr_re, &lines[index]);
        index += 1;

        program.push(instr);
    }

    (samples, program)
}

fn find_mapping(samples: &[(Regs, Instr, Regs)]) -> [i32; NUM_OPCODES] {
    // we build a matrix in which element (i, j) indicates if the given
    // opcode i could be mapping to the real opcode j. Eventually, this
    // matrix should contain exactly one "true" on each row, indicating
    // the given opcode can only be mapping to exactly one opcode.
    let mut matrix = [[true; NUM_OPCODES]; NUM_OPCODES];
    let mut mapping = [!0; NUM_OPCODES];

    // build matrix.
    for (before, instr, after) in samples.iter().cloned() {
        let [old, a, b, c] = instr;

        for new in 0..NUM_OPCODES {
            let valid = exec_instr(new as i32, a, b, c, before) == after;

            // Store if opcode "old" could be opcode "new"
            matrix[old as usize][new] &= valid;
        }
    }

    // check matrix.
    for _ in 0..NUM_OPCODES {
        let mut m = (!0, !0);

        for i in 0..NUM_OPCODES {
            print!("{} ", name_instr(i as i32));

            for j in 0..NUM_OPCODES {
                if matrix[j][i] || mapping[j] == i as i32 {
                    print!("x ");
                } else {
                    print!(". ");
                }
            }

            println!();
        }

        // find row with exactly one true element
        for i in 0..NUM_OPCODES {
            let count = matrix[i].iter().filter(|x| **x).count();
            let pos = matrix[i].iter().position(|x| *x);

            if count == 1 {
                m = (i, pos.unwrap());
            }
        }

        let (i, j) = m;
        mapping[i] = j as i32;
        println!(" mapping: {} = {}", name_instr(j as i32), i);
        println!();

        // clear row i and column j
        for k in 0..NUM_OPCODES {
            matrix[i][k] = false;
            matrix[k][j] = false;
        }
    }

    mapping
}

pub fn run(_: &[&str]) {
    let (samples, program) = parse_input();
    let mapping = find_mapping(&samples);

    let mut answer_a = 0;
    for (before, instr, after) in samples.iter().cloned() {
        let [_, a, b, c] = instr;
        let count = (0..NUM_OPCODES)
            .filter(|op| exec_instr(*op as i32, a, b, c, before) == after)
            .count();

        if count >= 3 {
            answer_a += 1;
        }
    }

    println!("answer A: {}", answer_a);

    let mut regs = Default::default();

    for [opcode, a, b, c] in program {
        regs = exec_instr(mapping[opcode as usize] as i32, a, b, c, regs);
    }

    println!("answer B: {:?}", regs);
}
