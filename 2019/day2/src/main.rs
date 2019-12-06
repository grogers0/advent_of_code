use std::fmt::Display;
use std::io::{self, Read};

use intcode::*;

fn part1(input: &str) -> impl Display {
    let mut program = parse(input);
    program[1] = 12;
    program[2] = 2;
    run(&mut program, None);
    program[0]
}

fn part2(input: &str) -> impl Display {
    let input_program = parse(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input_program.clone();
            program[1] = noun;
            program[2] = verb;
            run(&mut program, None);
            if program[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
