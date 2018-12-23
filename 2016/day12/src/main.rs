use std::collections::BTreeMap;
use std::io::{self, Read};

#[derive(Clone, Debug)]
enum Loc {
    Val(i64),
    Reg(String)
}

impl From<&str> for Loc {
    fn from(s: &str) -> Loc {
        if let Ok(val) = s.parse() {
            Loc::Val(val)
        } else {
            Loc::Reg(s.to_string())
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Cpy(Loc, String),
    Inc(String),
    Dec(String),
    Jnz(Loc, Loc)
}

fn parse(input: &str) -> Vec<Op> {
    input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "cpy" => Op::Cpy(Loc::from(tokens.next().unwrap()), tokens.next().unwrap().to_string()),
            "inc" => Op::Inc(tokens.next().unwrap().to_string()),
            "dec" => Op::Dec(tokens.next().unwrap().to_string()),
            "jnz" => Op::Jnz(Loc::from(tokens.next().unwrap()), Loc::from(tokens.next().unwrap())),
            _ => unreachable!()
        }
    })
    .collect()
}

fn lookup(loc: &Loc, registers: &BTreeMap<String, i64>) -> i64 {
    match loc {
        Loc::Val(x) => *x,
        Loc::Reg(x) => *registers.get(x).unwrap_or(&0)
    }
}

fn execute_op(op: &Op, pc: &mut i64, registers: &mut BTreeMap<String, i64>) {
    match op {
        Op::Cpy(x, y) => { registers.insert(y.clone(), lookup(x, registers)); },
        Op::Inc(x) => { registers.entry(x.to_string()).and_modify(|v| *v += 1).or_insert(1); },
        Op::Dec(x) => { registers.entry(x.to_string()).and_modify(|v| *v -= 1).or_insert(-1); },
        Op::Jnz(x, y) => if lookup(x, registers) != 0 { *pc += lookup(y, registers) - 1; }
    }
    *pc += 1;
}

fn part1(input: &str) -> i64 {
    let program = parse(input);
    let mut pc = 0i64;
    let mut registers = BTreeMap::new();
    while pc >= 0 && pc < program.len() as i64 {
        execute_op(&program[pc as usize], &mut pc, &mut registers);
    }
    *registers.get("a").unwrap_or(&0)
}

fn part2(input: &str) -> i64 {
    let program = parse(input);
    let mut pc = 0i64;
    let mut registers = BTreeMap::new();
    registers.insert("c".to_string(), 1);
    while pc >= 0 && pc < program.len() as i64 {
        execute_op(&program[pc as usize], &mut pc, &mut registers);
    }
    *registers.get("a").unwrap_or(&0)
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
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 42);
    }
}
