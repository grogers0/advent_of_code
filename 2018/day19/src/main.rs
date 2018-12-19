use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

use day16_2018::*;

type Instruction = (Op, usize, usize, usize);

fn parse_op(op_str: &str) -> Op {
    match op_str {
        "addr" => Op::Addr,
        "addi" => Op::Addi,
        "mulr" => Op::Mulr,
        "muli" => Op::Muli,
        "banr" => Op::Banr,
        "bani" => Op::Bani,
        "borr" => Op::Borr,
        "bori" => Op::Bori,
        "setr" => Op::Setr,
        "seti" => Op::Seti,
        "gtir" => Op::Gtir,
        "gtri" => Op::Gtri,
        "gtrr" => Op::Gtrr,
        "eqir" => Op::Eqir,
        "eqri" => Op::Eqri,
        "eqrr" => Op::Eqrr,
        "divr" => Op::Divr, // Need to optimize the loop to make it run faster
        _ => unreachable!()
    }
}

fn parse(input: &str) -> (usize, Vec<Instruction>) {
    lazy_static!{
        static ref IP_RE: Regex = Regex::new("^#ip (\\d+)$").unwrap();
        static ref INST_RE: Regex = Regex::new("^([a-z]+) (\\d+) (\\d+) (\\d+)$").unwrap();
    }
    let mut lines = input.lines();
    let cap = IP_RE.captures(lines.next().unwrap()).unwrap();
    let ip = cap[1].parse().unwrap();
    let instructions = lines.map(|line| {
        let cap = INST_RE.captures(line).unwrap();
        (parse_op(&cap[1]), cap[2].parse().unwrap(), cap[3].parse().unwrap(), cap[4].parse().unwrap())
    })
    .collect();
    (ip, instructions)
}

fn execute(ip: usize, program: &[Instruction], registers: &mut [usize]) {
    loop {
        let (op, a, b, c) = program[registers[ip]];
        execute_op(registers, op, a, b, c);
        if registers[ip] + 1 >= program.len() {
            break;
        }
        registers[ip] += 1;
    }
}

fn part1(input: &str) -> usize {
    let (ip, program) = parse(input);
    let mut registers = [0, 0, 0, 0, 0, 0];
    execute(ip, &program, &mut registers);
    registers[0]
}

fn part2(input: &str) -> usize {
    let (ip, program) = parse(input);
    let mut registers = [1, 0, 0, 0, 0, 0];
    execute(ip, &program, &mut registers);
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

    const EX: &str = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 6);
    }
}
