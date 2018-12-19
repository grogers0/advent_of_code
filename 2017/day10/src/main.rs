use std::fmt::Write;
use std::io::{self, Read};

use day10_2017::*;
use crate::reverse_circular::reverse_circular;

mod reverse_circular;

fn parse_lengths(input: &str) -> Vec<usize> {
    input.trim_end().split(",")
        .map(|len_str| len_str.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let lengths = parse_lengths(input);
    let mut list = Vec::new();
    for i in 0 ..= 255 { list.push(i) }

    let mut curr = 0;
    let mut skip = 0;
    for length in lengths {
        reverse_circular(&mut list, curr, length);
        curr += length + skip;
        skip += 1;
    }

    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let hash_bytes = knot_hash(input);

    let mut out = String::new();
    for byte in hash_bytes.iter() {
        write!(out, "{:02x}", byte).unwrap();
    }

    out
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272".to_string());
        assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd".to_string());
        assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d".to_string());
        assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e".to_string());
    }

}
