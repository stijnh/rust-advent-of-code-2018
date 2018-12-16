use crate::common::read_file_lines;
use regex::{Regex, Captures};

type Regs = [i32; 4];

fn parse_captures(re: &Regex, line: &str) -> [i32; 4] {
    if let Some(cap) = re.captures(line) {
        let mut output = [0; 4];

        let vec = cap.iter()
            .skip(1)
            .map(|x| x.unwrap().as_str())
            .map(|x| x.parse::<i32>().expect(x))
            .enumerate()
            .for_each(|(i, v)| output[i] = v);

        output
    } else {
        panic!("failed regex '{}' on '{}'",
               re.as_str(), line);
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

pub fn run(_: &[&str]) {
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
    };

    index += 2;

    while index < lines.len() {
        let instr = parse_captures(&instr_re, &lines[index]);
        index += 1;

        program.push(instr);
    }
    
    let mut answer_a = 0;
    for (before, instr, after) in samples.into_iter() {
        let [_, a, b, c] = instr;
        let count = (0..NUM_OPCODES)
            .filter(|op| exec_instr(*op as i32, a, b, c, before) == after)
            .count();

        if count >= 3 {
            answer_a += 1;
        }
    }

    println!("answer A: {}", answer_a);


}
