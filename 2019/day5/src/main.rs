use std::fmt::Display;
use std::io::{self, Read};
use std::sync::mpsc::channel;

use intcode::*;

fn part1(mem_str: &str) -> impl Display {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    tx_in.send(1).unwrap();
    run(&mut parse(mem_str), rx_in, tx_out);
    let mut outputs = Vec::new();
    while let Ok(val) = rx_out.recv() {
        outputs.push(val);
    }
    assert!(outputs.len() > 1);
    for val in &outputs[0..outputs.len()-1] {
        assert_eq!(0, *val);
    }
    outputs[outputs.len()-1]
}

fn part2(mem_str: &str) -> impl Display {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    tx_in.send(5).unwrap();
    run(&mut parse(mem_str), rx_in, tx_out);
    let output = rx_out.recv().unwrap();
    assert!(rx_out.recv().is_err());
    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
