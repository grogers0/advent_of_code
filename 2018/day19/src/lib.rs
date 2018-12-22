use lazy_static::lazy_static;
use regex::Regex;

pub use day16_2018::*;

pub type Instruction = (Op, usize, usize, usize);

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
        // Below here are needed for optimization
        "divr" => Op::Divr,
        "divi" => Op::Divi,
        _ => unreachable!()
    }
}

// (ip_register, instructions)
pub fn parse_instructions(input: &str) -> (usize, Vec<Instruction>) {
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

