use std::io::{self, Read};

use bit_vec::BitVec;

fn parse(input: &str) -> BitVec {
    let mut bits = BitVec::new();
    for ch in input.chars() {
        match ch {
            '0' => bits.push(false),
            '1' => bits.push(true),
            _ => unreachable!()
        }
    }
    bits
}

fn generate(len: usize, bits: &mut BitVec) {
    while bits.len() < len {
        bits.reserve(1 + bits.len());
        let mut copied = BitVec::with_capacity(bits.len());
        for bit in bits.iter().rev() {
            copied.push(!bit);
        }
        bits.push(false);
        bits.extend(copied.iter());
    }
    bits.truncate(len);
}

fn checksum(mut bits: BitVec) -> BitVec {
    while bits.len() % 2 == 0 {
        let mut checksum = BitVec::with_capacity(bits.len() / 2);
        let mut bits_iter = bits.into_iter();
        while let (Some(b1), Some(b2)) = (bits_iter.next(), bits_iter.next()) {
            checksum.push(b1 == b2);
        }
        bits = checksum;
    }
    bits
}

fn bits_to_string(bits: &BitVec) -> String {
    bits.iter().map(|b| if b { '1' } else { '0' }).collect()
}

fn calc(len: usize, input: &str) -> String {
    let mut bits = parse(input.trim_end());
    generate(len, &mut bits);
    let bits = checksum(bits);
    bits_to_string(&bits)
}

fn part1(input: &str) -> String {
    calc(272, input)
}

fn part2(input: &str) -> String {
    calc(35651584, input)
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
    fn test_checksum() {
        assert_eq!(checksum(parse("110010110100")), parse("100"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(&calc(20, "10000"), "01100");
    }
}
