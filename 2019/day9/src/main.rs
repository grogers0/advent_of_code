use std::io::{self, Read};
use std::sync::mpsc::channel;

use intcode::*;

fn run_with_input(mem_str: &str, input: i64) -> i64 {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    tx_in.send(input).unwrap();
    run(&mut parse(mem_str), &rx_in, tx_out);
    let output = rx_out.recv().unwrap();
    assert!(rx_out.recv().is_err());
    output
}

fn part1(input: &str) -> i64 {
    run_with_input(input, 1)
}

fn part2(input: &str) -> i64 {
    run_with_input(input, 2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
