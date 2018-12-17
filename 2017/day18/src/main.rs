use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Operand {
    Register(char),
    Value(i64)
}

#[derive(Copy, Clone, Debug)]
enum Cmd {
    Snd(Operand),
    Rcv(char),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Jgz(Operand, Operand)
}

fn parse_operand(op_str: &str) -> Operand {
    if let Ok(val) = op_str.parse() {
        Operand::Value(val)
    } else {
        Operand::Register(op_str.chars().next().unwrap())
    }
}

fn parse(input: &str) -> Vec<Cmd> {
    lazy_static!{
        static ref UNARY_RE: Regex = Regex::new("^(snd|rcv) ([a-z]|-?[0-9]+)$").unwrap();
        static ref BINARY_RE: Regex = Regex::new("^(set|add|mul|mod|jgz) ([a-z]|-?[0-9]+) ([a-z]|-?[0-9]+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            if let Some(cap) = UNARY_RE.captures(line) {
                if &cap[1] == "snd" {
                    Cmd::Snd(parse_operand(&cap[2]))
                } else if &cap[1] == "rcv" {
                    Cmd::Rcv(cap[2].chars().next().unwrap())
                } else {
                    unreachable!()
                }
            } else if let Some(cap) = BINARY_RE.captures(line) {
                let reg = cap[2].chars().next().unwrap();
                if &cap[1] == "set" {
                    Cmd::Set(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "add" {
                    Cmd::Add(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "mul" {
                    Cmd::Mul(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "mod" {
                    Cmd::Mod(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "jgz" {
                    Cmd::Jgz(parse_operand(&cap[2]), parse_operand(&cap[3]))
                } else {
                    unreachable!()
                } 
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn value_of(registers: &mut BTreeMap<char, i64>, operand: Operand) -> i64 {
    match operand {
        Operand::Value(val) => val,
        Operand::Register(x) => *registers.get(&x).unwrap_or(&0)
    }
}

fn execute_part1(cmd: &Cmd, registers: &mut BTreeMap<char, i64>, pc: &mut i64, sound: &mut i64, recovered: &mut Option<i64>) {
    *pc += 1;
    match *cmd {
        Cmd::Snd(x) => *sound = value_of(registers, x),
        Cmd::Rcv(x) => if *registers.get(&x).unwrap_or(&0) != 0 { *recovered = Some(*sound) },
        Cmd::Set(x, y) => {
            let val = value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Add(x, y) => {
            let val = (*registers.get(&x).unwrap_or(&0)).checked_add(value_of(registers, y)).unwrap();
            registers.insert(x, val);
        },
        Cmd::Mul(x, y) => {
            let val = (*registers.get(&x).unwrap_or(&0)).checked_mul(value_of(registers, y)).unwrap();
            registers.insert(x, val);
        },
        Cmd::Mod(x, y) => {
            let val = *registers.get(&x).unwrap_or(&0) % value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Jgz(x, y) => if value_of(registers, x) > 0 { *pc += value_of(registers, y) - 1; }
    }
}

fn part1(input: &str) -> i64 {
    let instructions = parse(input);
    let mut registers = BTreeMap::new();

    let mut pc = 0;
    let mut sound = 0;
    let mut recovered = None;
    while recovered.is_none() {
        execute_part1(&instructions[pc as usize], &mut registers, &mut pc, &mut sound, &mut recovered);
    }
    recovered.unwrap()
}

fn execute_part2(cmd: &Cmd, registers: &mut BTreeMap<char, i64>, pc: &mut i64, sndbuf: &mut VecDeque<i64>, rcvbuf: &mut VecDeque<i64>) -> bool {
    match *cmd {
        Cmd::Snd(x) => sndbuf.push_back(value_of(registers, x)),
        Cmd::Rcv(x) => {
            if rcvbuf.is_empty() {
                return false
            } else {
                registers.insert(x, rcvbuf.pop_front().unwrap());
            }
        },
        Cmd::Set(x, y) => {
            let val = value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Add(x, y) => {
            let val = *registers.get(&x).unwrap_or(&0) + value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Mul(x, y) => {
            let val = *registers.get(&x).unwrap_or(&0) * value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Mod(x, y) => {
            if *registers.get(&x).unwrap_or(&0) < 0 || value_of(registers, y) < 0 { panic!() } // Any standard way to handle negatives?
            let val = *registers.get(&x).unwrap_or(&0) % value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Jgz(x, y) => if value_of(registers, x) > 0 { *pc += value_of(registers, y) - 1; }
    }
    *pc += 1;
    true
}

fn part2(input: &str) -> usize {
    let instructions = parse(input);
    let mut registers0 = BTreeMap::new();
    registers0.insert('p', 0);
    let mut registers1 = BTreeMap::new();
    registers1.insert('p', 1);

    let mut pc0 = 0;
    let mut pc1 = 0;
    let mut rcvbuf0 = VecDeque::new();
    let mut rcvbuf1 = VecDeque::new();
    let mut send_count = 0;
    loop {
        if execute_part2(&instructions[pc0 as usize], &mut registers0, &mut pc0, &mut rcvbuf1, &mut rcvbuf0) {
            continue
        }
        let start_len = rcvbuf0.len();
        if execute_part2(&instructions[pc1 as usize], &mut registers1, &mut pc1, &mut rcvbuf0, &mut rcvbuf1) {
            if start_len != rcvbuf0.len() {
                send_count += 1;
            }
            continue
        }
        break; // Deadlock
    }
    send_count
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

    const EX1: &str = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 4);
    }

    const EX2: &str = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX2), 3);
    }

}
