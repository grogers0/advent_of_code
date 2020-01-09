use std::fmt::Display;
use std::io::{self, Read};
use std::sync::mpsc::channel;
use std::thread;

use intcode::*;

fn springdroid(mut mem: Mem, commands: &str) -> i64 {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    thread::spawn(move || run(&mut mem, &rx_in, tx_out));

    send_line(&tx_in, commands);

    let mut debug_output = String::new();
    while let Ok(val) = rx_out.recv() {
        if val >= 128 {
            return val
        } else {
            debug_output.push(val as u8 as char);
        }
    }
    eprintln!("{}", debug_output);
    panic!()
}

// Jump if: we would jump over a hole, and can land safely.
fn part1(mem_str: &str) -> impl Display {
    springdroid(parse(mem_str),
    // !(A && B && C) && D
"OR A J
AND B J
AND C J
NOT J J
AND D J
WALK")
}

// Jump if: we would jump over a hole, and can land safely, and we can leave safely after.
fn part2(mem_str: &str) -> impl Display {
    springdroid(parse(mem_str),
    // !(A && B && C) && D && (H || E)
    // NOTE - It may seem like E should be (E && (F || I)) but in any case where (!F && !H && !I)
    // but is still traversable at all, we would have already jumped:
    //    ##xx###.#..x#
    // > ##xx###.#..x#
    //    @ABCDEFGHI
"OR A J
AND B J
AND C J
NOT J J
AND D J
OR H T
OR E T
AND T J
RUN")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
