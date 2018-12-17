use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Operand {
    Register(char),
    Value(i64)
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        match *self {
            Operand::Register(x) => write!(f, "{}", x),
            Operand::Value(x) => write!(f, "{}", x)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Cmd {
    Set(char, Operand),
    Sub(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Jnz(Operand, Operand),
    Nop
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        match *self {
            Cmd::Set(x, y) => write!(f, "set {} {}", x, y),
            Cmd::Sub(x, y) => write!(f, "sub {} {}", x, y),
            Cmd::Mul(x, y) => write!(f, "mul {} {}", x, y),
            Cmd::Mod(x, y) => write!(f, "mod {} {}", x, y),
            Cmd::Jnz(x, y) => write!(f, "jnz {} {}", x, y),
            Cmd::Nop => write!(f, "nop")
        }
    }
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
        static ref BINARY_RE: Regex = Regex::new("^(set|sub|mul|jnz) ([a-z]|-?[0-9]+) ([a-z]|-?[0-9]+)$").unwrap();
    }
    input.lines()
        .map(|line| {
            if let Some(cap) = BINARY_RE.captures(line) {
                let reg = cap[2].chars().next().unwrap();
                if &cap[1] == "set" {
                    Cmd::Set(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "sub" {
                    Cmd::Sub(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "mul" {
                    Cmd::Mul(reg, parse_operand(&cap[3]))
                } else if &cap[1] == "jnz" {
                    Cmd::Jnz(parse_operand(&cap[2]), parse_operand(&cap[3]))
                } else {
                    unreachable!()
                } 
            } else {
                println!("{}", line);
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

fn execute(cmd: &Cmd, registers: &mut BTreeMap<char, i64>, pc: &mut i64) {
    *pc += 1;
    match *cmd {
        Cmd::Set(x, y) => {
            let val = value_of(registers, y);
            registers.insert(x, val);
        },
        Cmd::Sub(x, y) => {
            let val = (*registers.get(&x).unwrap_or(&0)).checked_sub(value_of(registers, y)).unwrap();
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
        Cmd::Jnz(x, y) => if value_of(registers, x) != 0 { *pc += value_of(registers, y) - 1; },
        Cmd::Nop => ()
    }
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let mut registers = BTreeMap::new();

    let mut pc = 0i64;
    let mut mul_count = 0;
    while pc >= 0 && pc < instructions.len() as i64 {
        let cmd = &instructions[pc as usize];
        if let Cmd::Mul(_, _) = cmd {
            mul_count += 1;
        }
        execute(&cmd, &mut registers, &mut pc);
    }
    mul_count
}

fn part2(input: &str) -> i64 {
    let mut instructions = parse(input);
    // This loop used to be calculating the modulo (b % d) by trying every value e in 2..b and
    // checking if b=d*e, aka if b%d=0, and setting f=0 if so
    instructions[10] = Cmd::Set('g', Operand::Register('b'));
    instructions[11] = Cmd::Mod('g', Operand::Register('d'));
    instructions[12] = Cmd::Jnz(Operand::Register('g'), Operand::Value(8));
    instructions[13] = Cmd::Set('f', Operand::Value(0));
    instructions[14] = Cmd::Jnz(Operand::Value(1), Operand::Value(11));
    instructions[15] = Cmd::Nop;
    instructions[16] = Cmd::Nop;
    instructions[17] = Cmd::Nop;
    instructions[18] = Cmd::Nop;
    instructions[19] = Cmd::Nop;
    let instructions = instructions;
    let mut registers = BTreeMap::new();
    registers.insert('a', 1);

    let mut pc = 0i64;
    while pc >= 0 && pc < instructions.len() as i64 {
        let cmd = &instructions[pc as usize];
        execute(&cmd, &mut registers, &mut pc);
    }

    *registers.get(&'h').unwrap_or(&0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
