use std::io::{self, Read};

use day19_2018::*;

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
    let (ip, program) = parse_instructions(input);
    let mut registers = [0, 0, 0, 0, 0, 0];
    execute(ip, &program, &mut registers);
    registers[0]
}

fn part2(input: &str) -> usize {
    let (ip, program) = parse_instructions(input);
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
