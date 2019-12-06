use std::fmt::Display;
use std::io::{self, Read};

use intcode::*;

fn part1(input: &str) -> impl Display {
    let mut mem = parse(input);
    let output = run(&mut mem, Some(1));
    assert!(output.len() > 1);
    for outval in &output[0 .. (output.len() - 1)] {
        assert_eq!(0, *outval);
    }
    output[output.len() - 1]
}

fn part2(input: &str) -> impl Display {
    let mut mem = parse(input);
    let output = run(&mut mem, Some(5));
    assert_eq!(1, output.len());
    output[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
