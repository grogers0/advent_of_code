use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Op {
    Acc, Jmp, Nop
}
#[derive(Copy, Clone)]
struct Instruction {
    op: Op,
    arg: i32
}

fn parse(puzzle_input: &str) -> Vec<Instruction> {
    puzzle_input.lines().map(|line| {
        let mut words = line.split(char::is_whitespace);
        let op = match words.next().unwrap() {
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            "nop" => Op::Nop,
            _ => panic!()
        };
        let arg = words.next().unwrap().parse().unwrap();
        assert!(words.next().is_none());
        Instruction { op: op, arg: arg }
    }).collect()
}

enum ExecutionResult {
    Terminated(i32),
    InfiniteLoop(i32)
}

fn execute(instructions: &Vec<Instruction>) -> ExecutionResult {
    let mut acc = 0;
    let mut pc = 0;
    let mut seen = HashSet::new();
    loop {
        if pc as usize == instructions.len() {
            return ExecutionResult::Terminated(acc)
        } else if pc as usize > instructions.len() {
            panic!()
        } else if !seen.insert(pc) {
            return ExecutionResult::InfiniteLoop(acc)
        }
        let curr = &instructions[pc as usize];
        match curr.op {
            Op::Acc => { acc += curr.arg; pc += 1 },
            Op::Jmp => pc += curr.arg,
            Op::Nop => pc += 1
        }
    }
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    match execute(instructions) {
        ExecutionResult::Terminated(_) => panic!(),
        ExecutionResult::InfiniteLoop(acc) => acc,
    }
}

fn part2(orig_instructions: &Vec<Instruction>) -> i32 {
    let mut instructions = orig_instructions.clone();
    for i in 0..orig_instructions.len() {
        instructions[i] = match orig_instructions[i] {
            Instruction { op: Op::Jmp, arg } => Instruction { op: Op::Nop, arg: arg },
            Instruction { op: Op::Nop, arg } => Instruction { op: Op::Jmp, arg: arg },
            _ => continue
        };

        if let ExecutionResult::Terminated(acc) = execute(&instructions) {
            return acc
        }

        instructions[i] = orig_instructions[i];
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let instructions = parse(&puzzle_input);

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2(&parse(EX)));
    }
}
