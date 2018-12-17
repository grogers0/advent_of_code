use std::fmt::Write;
use std::io::{self, Read};

fn parse_lengths(input: &str) -> Vec<usize> {
    input.trim_end().split(",")
        .map(|len_str| len_str.parse().unwrap())
        .collect()
}

pub fn reverse_circular<T: Copy>(list: &mut Vec<T>, mut start: usize, mut len: usize) {
    while len > 1 {
        let tmp = list[start % 256];
        list[start % 256] = list[(start + len - 1) % 256];
        list[(start + len - 1) % 256] = tmp;
        start += 1;
        len -= 2;
    }
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

fn parse_lengths_as_ascii(input: &str) -> Vec<usize> {
    let mut lengths: Vec<usize> = input.trim_end().chars()
        .map(|ch| ch as usize)
        .collect();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);
    lengths
}

pub fn knot_hash(input: &str) -> Vec<u8> {
    let lengths = parse_lengths_as_ascii(input);
    let mut list: Vec<u8> = Vec::new();
    for i in 0 ..= 255 { list.push(i) }

    let mut curr = 0;
    let mut skip = 0;
    for _round in 0..64 {
        for length in &lengths {
            reverse_circular(&mut list, curr, *length);
            curr += *length + skip;
            skip += 1;
        }
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc,x| acc ^ x))
        .collect()
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
