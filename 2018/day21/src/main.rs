use std::collections::BTreeSet;
use std::io::{self, Read};

use day19_2018::*;

fn part1(input: &str) -> usize {
    let (ip, program) = parse_instructions(input);
    let mut registers = [0, 0, 0, 0, 0, 0];
    loop {
        let (op, a, b, c) = program[registers[ip]];
        // The prompt asks to find the value of r0 which minimizes the instructions executed to get
        // to the halt point. This eqrr instruction is what causes the halt so we can get the value
        // directly from what is being compared to r0.
        if op == Op::Eqrr && b == 0  {
            return registers[a];
        }
        execute_op(&mut registers, op, a, b, c);
        registers[ip] += 1;
    }
}

// NOTE - must be run with input_optimized, see README
fn part2(input: &str) -> usize {
    let (ip, program) = parse_instructions(input);
    let mut registers = [0, 0, 0, 0, 0, 0];
    let mut seen = BTreeSet::new();
    let mut last_val = 0;
    loop {
        let (op, a, b, c) = program[registers[ip]];
        if op == Op::Eqrr && b == 0  {
            if !seen.insert(registers[a]) {
                return last_val;
            }
            last_val = registers[a];
            execute_op(&mut registers, Op::Seti, 0, b, c);
        } else {
            execute_op(&mut registers, op, a, b, c);
        }
        registers[ip] += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
