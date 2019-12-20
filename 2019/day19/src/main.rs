use std::io::{self, Read};
use std::sync::mpsc::channel;

use intcode::*;

fn is_tractor(mut mem: Mem, x: usize, y: usize) -> bool {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    tx_in.send(x as i64).unwrap();
    tx_in.send(y as i64).unwrap();

    run(&mut mem, rx_in, tx_out);

    match rx_out.recv().unwrap() {
        0 => false,
        1 => true,
        _ => panic!()
    }
}

struct TopEdgeIter {
    x: usize,
    y: usize,
    mem: Mem
}

impl TopEdgeIter {
    fn new(mem: Mem, start_y: usize) -> Self {
        let mut found = false;
        for x in 0 .. {
            if is_tractor(mem.clone(), x, start_y) {
                found = true
            } else if found {
                return Self {
                    x: x - 1,
                    y: start_y,
                    mem: mem
                }
            }
        }
        unreachable!()
    }
}

impl Iterator for TopEdgeIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        while !is_tractor(self.mem.clone(), self.x, self.y) {
            self.y += 1;
        }
        Some((self.x, self.y))
    }
}

fn part1(mem_str: &str) -> usize {
    let mem = parse(mem_str);
    let mut sum = 0;
    for y in 0 .. 50 {
        for x in 0 .. 50 {
            if is_tractor(mem.clone(), x, y) {
                sum += 1
            }
        }
    }
    sum
}

fn part2(mem_str: &str) -> usize {
    let mem = parse(mem_str);
    let side_len = 100;
    let mut top_edge = TopEdgeIter::new(mem.clone(), side_len);
    loop {
        let (right_x, top_y) = top_edge.next().unwrap();
        if right_x + 1 < side_len { continue }
        let left_x = right_x + 1 - side_len;
        let bottom_y = top_y + side_len - 1;
        if is_tractor(mem.clone(), left_x, bottom_y) {
            return left_x * 10000 + top_y;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
