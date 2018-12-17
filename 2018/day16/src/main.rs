use std::collections::BTreeSet;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Addr, Addi, // C = rA + (r/v)B
    Mulr, Muli, // C = rA * (r/v)B
    Banr, Bani, // C = rA & (r/v)B
    Borr, Bori, // C = rA | (r/v)B
    Setr, Seti, // C = (r/v)A // B ignored
    Gtir, Gtri, Gtrr, // if (r/v)A > (r/v)B { C = 1 } else { C = 0 }
    Eqir, Eqri, Eqrr // if (r/v)A == (r/v)B { C = 1 } else { C = 0 }
}

#[derive(Clone, Debug)]
struct OpcodeSample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4]
}

fn parse(input: &str) -> (Vec<OpcodeSample>, Vec<[usize; 4]>) {
    lazy_static!{
        static ref BEFORE_RE: Regex = Regex::new("^Before: \\[(\\d+), (\\d+), (\\d+), (\\d+)\\]$").unwrap();
        static ref INSTRUCTION_RE: Regex = Regex::new("^(\\d+) (\\d+) (\\d+) (\\d+)$").unwrap();
        static ref AFTER_RE: Regex = Regex::new("^After:  \\[(\\d+), (\\d+), (\\d+), (\\d+)\\]$").unwrap();
    }

    let mut samples = Vec::new();
    let mut program = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if let Some(cap) = BEFORE_RE.captures(line) {
            let before = [cap[1].parse().unwrap(), cap[2].parse().unwrap(),
                          cap[3].parse().unwrap(), cap[4].parse().unwrap()];
            let cap = INSTRUCTION_RE.captures(lines.next().unwrap()).unwrap();
            let instruction = [cap[1].parse().unwrap(), cap[2].parse().unwrap(),
                               cap[3].parse().unwrap(), cap[4].parse().unwrap()];
            let cap = AFTER_RE.captures(lines.next().unwrap()).unwrap();
            let after = [cap[1].parse().unwrap(), cap[2].parse().unwrap(),
                         cap[3].parse().unwrap(), cap[4].parse().unwrap()];
            samples.push(OpcodeSample {
                before: before,
                instruction: instruction,
                after: after
            });
        } else if let Some(cap) = INSTRUCTION_RE.captures(line) {
            let instruction = [cap[1].parse().unwrap(), cap[2].parse().unwrap(),
                               cap[3].parse().unwrap(), cap[4].parse().unwrap()];
            program.push(instruction);
        } else if line == "" {
            // Skip
        } else {
            unreachable!()
        }
    }
    (samples, program)
}

fn execute_decoded(registers: &mut [usize; 4], op: Op, a: usize, b: usize, c: usize) {
    match op {
        Op::Addr => registers[c] = registers[a] + registers[b],
        Op::Addi => registers[c] = registers[a] + b,
        Op::Mulr => registers[c] = registers[a] * registers[b],
        Op::Muli => registers[c] = registers[a] * b,
        Op::Banr => registers[c] = registers[a] & registers[b],
        Op::Bani => registers[c] = registers[a] & b,
        Op::Borr => registers[c] = registers[a] | registers[b],
        Op::Bori => registers[c] = registers[a] | b,
        Op::Setr => registers[c] = registers[a],
        Op::Seti => registers[c] = a,
        Op::Gtir => registers[c] = if a > registers[b] { 1 } else { 0 },
        Op::Gtri => registers[c] = if registers[a] > b { 1 } else { 0 },
        Op::Gtrr => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
        Op::Eqir => registers[c] = if a == registers[b] { 1 } else { 0 },
        Op::Eqri => registers[c] = if registers[a] == b { 1 } else { 0 },
        Op::Eqrr => registers[c] = if registers[a] == registers[b] { 1 } else { 0 }
    }
}

fn all_ops() -> BTreeSet<Op> {
    vec![Op::Addr, Op::Addi, Op::Mulr, Op::Muli, Op::Banr, Op::Bani, Op::Borr,
         Op::Bori, Op::Setr, Op::Seti, Op::Gtir, Op::Gtri, Op::Gtrr, Op::Eqir,
         Op::Eqri, Op::Eqrr].into_iter().collect()
}

fn possible_ops(before: [usize; 4], after: [usize; 4], [_, a, b, c]: [usize; 4]) -> BTreeSet<Op> {
    all_ops().into_iter().filter(|op| {
        let mut registers = before.clone();
        execute_decoded(&mut registers, *op, a, b, c);
        registers == after
    }).collect()
}

fn part1(input: &str) -> usize {
    let (samples, _) = parse(input);
    samples.into_iter()
        .filter(|sample| possible_ops(sample.before, sample.after, sample.instruction).len() >= 3)
        .count()
}

fn determine_opcodes(samples: &Vec<OpcodeSample>) -> Vec<Op> {
    let mut opcode_possibilities = (0..all_ops().len())
        .map(|_| all_ops()).collect::<Vec<_>>();
    for sample in samples.iter() {
        let ops = possible_ops(sample.before, sample.after, sample.instruction);
        opcode_possibilities[sample.instruction[0]] =
            opcode_possibilities[sample.instruction[0]].intersection(&ops).into_iter().cloned().collect();
    }
    let mut opcode_options = vec![None; all_ops().len()];
    while opcode_possibilities.iter().any(|ops| ops.len() > 0) {
        for opcode in 0..all_ops().len() {
            if opcode_possibilities[opcode].len() == 0 {
                assert!(opcode_options[opcode].is_some());
            } else if opcode_possibilities[opcode].len() == 1 {
                let op = *opcode_possibilities[opcode].iter().next().unwrap();
                opcode_options[opcode] = Some(op);
                for ops in opcode_possibilities.iter_mut() {
                    ops.remove(&op);
                }
            }
        }
    }
    opcode_options.into_iter().map(|opt| opt.unwrap()).collect()
}

fn part2(input: &str) -> usize {
    let (samples, program) = parse(input);
    let opcodes = determine_opcodes(&samples);
    let mut registers = [0, 0, 0, 0];

    for [opcode, a, b, c] in program {
        let op = opcodes[opcode];
        execute_decoded(&mut registers, op, a, b, c);
    }

    registers[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_ops() {
        let ex = "\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        let sample = parse(ex).0[0].clone();
        assert_eq!(possible_ops(sample.before, sample.after, sample.instruction),
                   [Op::Mulr, Op::Addi, Op::Seti].into_iter().cloned().collect());
    }
}
