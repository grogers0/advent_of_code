use std::io::{self, Read};

fn parse(puzzle_input: &str) -> (usize, Vec<u32>) {
    let n_bits = puzzle_input.lines().next().unwrap().chars().count();
    assert!(n_bits <= u32::BITS as usize);
    let diags = puzzle_input.lines().map(|line| {
        assert_eq!(n_bits, line.chars().count());
        let mut num = 0;
        for ch in line.chars() {
            num = num << 1;
            match ch {
                '0' => (),
                '1' => num |= 1,
                _ => panic!()
            };
        }
        num 
    }).collect();
    (n_bits, diags)
}

fn count_ones_in_bit_pos(diags: &[u32], bit_pos: usize) -> usize {
    diags.iter().filter(|diag| (*diag & (1 << bit_pos)) != 0).count()
}

fn most_common_in_bit_pos(diags: &[u32], bit_pos: usize) -> u32 {
    if count_ones_in_bit_pos(diags, bit_pos) * 2 >= diags.len() { 1 } else { 0 }
}

fn least_common_in_bit_pos(diags: &[u32], bit_pos: usize) -> u32 {
    if count_ones_in_bit_pos(diags, bit_pos) * 2 < diags.len() { 1 } else { 0 }
}

fn part1(n_bits: usize, diags: &[u32]) -> u32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..n_bits {
        if most_common_in_bit_pos(diags, i) == 1 {
            gamma_rate |= 1 << i;
        }
        if least_common_in_bit_pos(diags, i) == 1 {
            epsilon_rate |= 1 << i;
        }
    }
    gamma_rate * epsilon_rate
}

fn select_with_bit_criteria(n_bits: usize, diags: &[u32], bit_criteria: fn(diags: &[u32], bit_pos: usize) -> u32) -> u32 {
    let mut diags = diags.to_vec();
    let mut bit_pos = n_bits - 1;
    while diags.len() > 1 {
        let desired = bit_criteria(&diags, bit_pos);
        diags.retain(|diag| (*diag & (1 << bit_pos)) == (desired << bit_pos));
        if bit_pos == 0 { break } else { bit_pos -= 1 }
    }
    *diags.iter().next().unwrap()
}

fn part2(n_bits: usize, diags: &[u32]) -> u32 {
    let o2_rating = select_with_bit_criteria(n_bits, diags, most_common_in_bit_pos);
    let co2_rating = select_with_bit_criteria(n_bits, diags, least_common_in_bit_pos);
    o2_rating * co2_rating
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (n_bits, diags) = parse(&puzzle_input);
    println!("{}", part1(n_bits, &diags));
    println!("{}", part2(n_bits, &diags));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let (n_bits, diags) = parse(EX);
        assert_eq!(198, part1(n_bits, &diags));
    }

    #[test]
    fn test_part2() {
        let (n_bits, diags) = parse(EX);
        assert_eq!(230, part2(n_bits, &diags));
    }
}
