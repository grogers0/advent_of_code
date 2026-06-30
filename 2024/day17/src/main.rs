use std::io::{self, Read};
use regex::Regex;

#[derive(Copy, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

type Program = Vec<u8>;

fn parse(puzzle_input: &str) -> (Registers, Program) {
    let re = Regex::new(
        r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([0-9,]+)").unwrap();
    let cap = re.captures(puzzle_input).unwrap();
    let a = cap[1].parse::<u64>().unwrap();
    let b = cap[2].parse::<u64>().unwrap();
    let c = cap[3].parse::<u64>().unwrap();
    let registers = Registers { a, b, c };
    let program = cap[4].split(",").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>();
    for &v in &program {
        assert!(v <= 7);
    }
    assert!(program.len() % 2 == 0);
    (registers, program)
}

fn combo(registers: &Registers, op: u8) -> u64 {
    match op {
        0 | 1 | 2 | 3 => op as u64,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!(),
    }
}

fn execute(mut registers: Registers, program: &Program) -> Program {
    let mut output = Vec::with_capacity(program.len());
    let mut pc = 0;
    while pc < program.len() {
        let inst = program[pc];
        let literal_op = program[pc + 1] as u64;
        match inst {
            0 /* adv (division) */ => {
                let combo_op = combo(&registers, program[pc + 1]);
                registers.a = registers.a / (1 << combo_op);
            },
            1 /* bxl */ => {
                registers.b = registers.b ^ literal_op;
            },
            2 /* bst */ => {
                let combo_op = combo(&registers, program[pc + 1]);
                registers.b = combo_op % 8;
            },
            3 /* jnz */ => {
                if registers.a != 0 {
                    pc = literal_op.try_into().unwrap();
                    continue;
                }
            },
            4 /* bxc */ => {
                registers.b = registers.b ^ registers.c;
            },
            5 /* out */ => {
                let combo_op = combo(&registers, program[pc + 1]);
                output.push((combo_op % 8) as u8);
            },
            6 /* bdv */ => {
                let combo_op = combo(&registers, program[pc + 1]);
                registers.b = registers.a / (1 << combo_op);
            },
            7 /* cdv */ => {
                let combo_op = combo(&registers, program[pc + 1]);
                registers.c = registers.a / (1 << combo_op);
            },
            _ => panic!(),
        }
        pc += 2;
    }
    output
}

fn part1(registers: Registers, program: &Program) -> String {
    let output = execute(registers, program);
    let mut output_str = String::with_capacity(output.len() * 2 + 1);
    let mut first = true;
    for v in output {
        if first {
            first = false;
        } else {
            output_str.push(',');
        }
        output_str.push_str(&v.to_string());
    }
    output_str
}

fn part2(registers: Registers, program: &Program) -> u64 {
    // Won't work in general, but in both the example and the given input, the iteration output
    // only depends on the value of a. Both b and c are overwritten on each iteration, so we can
    // search for what the execution outputs and match against the suffix of the program as the
    // program itself shifts a down each iteration.
    fn search(mut registers: Registers, program: &Program, prefix: u64) -> Option<u64> {
        let start = if prefix == 0 { 1 } else { 0 };
        for suffix in start..256 {
            let curr = (prefix << 3) + suffix;
            registers.a = curr;
            let output = execute(registers, program);
            if output.len() <= program.len() && output == program[(program.len() - output.len())..] {
                if output.len() == program.len() {
                    return Some(curr);
                } else if let Some(ret) = search(registers, program, curr) {
                    return Some(ret);
                }
            }
        }
        None
    }
    search(registers, program, 0).unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (registers, program) = parse(&puzzle_input);
    println!("{}", part1(registers, &program));
    println!("{}", part2(registers, &program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EX2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part1() {
        let (registers, program) = parse(EX1);
        assert_eq!(part1(registers, &program), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let (registers, program) = parse(EX2);
        assert_eq!(part2(registers, &program), 117440);
    }
}
