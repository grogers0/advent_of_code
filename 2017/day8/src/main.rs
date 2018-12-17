use std::collections::BTreeMap;
use std::cmp::max;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Condition {
    register: String,
    op: String,
    value: i32
}

#[derive(Debug)]
struct Instruction {
    register: String,
    op: String,
    value: i32,
    condition: Condition
}

fn parse(input: &str) -> Vec<Instruction> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^([a-z]+) (inc|dec) ([0-9-]+) if ([a-z]+) (>|>=|<|<=|==|!=) ([0-9-]+)$").unwrap();
    }

    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Instruction {
                register: cap[1].to_string(),
                op: cap[2].to_string(),
                value: cap[3].parse().unwrap(),
                condition: Condition {
                    register: cap[4].to_string(),
                    op: cap[5].to_string(),
                    value: cap[6].parse().unwrap()
                }
            }
        })
        .collect()
}

fn meets_condition(registers: &BTreeMap<String, i32>, cond: &Condition) -> bool {
    let reg_value = registers.get(&cond.register).map(|x| *x).unwrap_or(0);
    if      cond.op == "<"  { reg_value <  cond.value }
    else if cond.op == "<=" { reg_value <= cond.value }
    else if cond.op == ">"  { reg_value >  cond.value }
    else if cond.op == ">=" { reg_value >= cond.value }
    else if cond.op == "==" { reg_value == cond.value }
    else if cond.op == "!=" { reg_value != cond.value }
    else { unreachable!() }
}

fn execute(registers: &mut BTreeMap<String, i32>, inst: &Instruction) {
    registers.entry(inst.register.clone()).or_insert(0);
    if inst.op == "inc" {
        registers.entry(inst.register.clone()).and_modify(|r| *r += inst.value);
    } else if inst.op == "dec" {
        registers.entry(inst.register.clone()).and_modify(|r| *r -= inst.value);
    } else {
        unreachable!();
    }
}

fn part1(input: &str) -> i32 {
    let instructions = parse(input);
    let mut registers = BTreeMap::new();

    for inst in instructions.iter() {
        if meets_condition(&registers, &inst.condition) {
            execute(&mut registers, &inst);
        }
    }

    *registers.values().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let instructions = parse(input);
    let mut registers = BTreeMap::new();
    let mut max_value = 0;

    for inst in instructions.iter() {
        if meets_condition(&registers, &inst.condition) {
            execute(&mut registers, &inst);
            max_value = max(max_value, *registers.get(&inst.register).unwrap());
        }
    }

    max_value
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
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 10);
    }
}
