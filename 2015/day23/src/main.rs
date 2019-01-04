use std::collections::BTreeMap;
use std::io::{self, Read};

use regex::Regex;

enum Op {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize)
}

fn parse_program(input: &str) -> Vec<Op> {
    let unary_re = Regex::new("^(hlf|tpl|inc) ([ab])$").unwrap();
    let jmp_re = Regex::new("^jmp ([+-]\\d+)$").unwrap();
    let binary_re = Regex::new("^(jie|jio) ([ab]), ([+-]\\d+)$").unwrap();
    input.lines()
        .map(|line| {
            if let Some(cap) = unary_re.captures(line) {
                let reg = cap[2].chars().next().unwrap();
                match &cap[1] {
                    "hlf" => Op::Hlf(reg),
                    "tpl" => Op::Tpl(reg),
                    "inc" => Op::Inc(reg),
                    _ => panic!()
                }
            } else if let Some(cap) = jmp_re.captures(line) {
                let offset = cap[1].parse().unwrap();
                Op::Jmp(offset)
            } else if let Some(cap) = binary_re.captures(line) {
                let reg = cap[2].chars().next().unwrap();
                let offset = cap[3].parse().unwrap();
                match &cap[1] {
                    "jie" => Op::Jie(reg, offset),
                    "jio" => Op::Jio(reg, offset),
                    _ => panic!()
                }
            } else {
                panic!();
            }
        })
        .collect()
}

fn execute_op(op: &Op, pc: &mut isize, registers: &mut BTreeMap<char, u64>) {
    match op {
        Op::Hlf(reg) => *registers.get_mut(reg).unwrap() /= 2,
        Op::Tpl(reg) => *registers.get_mut(reg).unwrap() *= 3,
        Op::Inc(reg) => *registers.get_mut(reg).unwrap() += 1,
        Op::Jmp(offset) => *pc += offset - 1,
        Op::Jie(reg, offset) => if registers[reg] % 2 == 0 { *pc += offset - 1; },
        Op::Jio(reg, offset) => if registers[reg] == 1 { *pc += offset - 1; }
    }
    *pc += 1;
}

fn execute_to_completion(program: &[Op], registers: &mut BTreeMap<char, u64>) {
    let mut pc = 0;
    while pc >= 0 && (pc as usize) < program.len() {
        execute_op(&program[pc as usize], &mut pc, registers);
    }
}

fn part1(input: &str) -> u64 {
    let program = parse_program(input);
    let mut registers = vec![('a', 0), ('b', 0)].into_iter().collect();
    execute_to_completion(&program, &mut registers);
    registers[&'b']
}

fn part2(input: &str) -> u64 {
    let program = parse_program(input);
    let mut registers = vec![('a', 1), ('b', 0)].into_iter().collect();
    execute_to_completion(&program, &mut registers);
    registers[&'b']
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

    const EX: &str = "\
inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn test_part1() {
        let program = parse_program(EX);
        let mut registers = vec![('a', 0), ('b', 0)].into_iter().collect();
        execute_to_completion(&program, &mut registers);
        assert_eq!(registers[&'a'], 2);
    }
}
