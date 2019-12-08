use std::fmt::Display;
use std::io::{self, Read};
use std::sync::mpsc::channel;

use intcode::*;

fn part1(mem_str: &str) -> impl Display {
    let mut mem = parse(mem_str);
    mem[1] = 12;
    mem[2] = 2;
    run(&mut mem, channel().1, channel().0);
    mem[0]
}

fn part2(mem_str: &str) -> impl Display {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = parse(mem_str);
            mem[1] = noun;
            mem[2] = verb;
            run(&mut mem, channel().1, channel().0);
            if mem[0] == 19690720 {
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
