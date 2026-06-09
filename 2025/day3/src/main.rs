use std::io::{self, Read};
use std::collections::HashMap;

fn parse(puzzle_input: &str) -> Vec<Vec<u8>> {
    let mut ret = vec![];
    for line in puzzle_input.lines() {
        let mut bank = vec![];
        for ch in line.chars() {
            bank.push(ch.to_digit(10).unwrap() as u8);
        }
        ret.push(bank);
    }
    ret
}

fn part1(banks: &[Vec<u8>]) -> u64 {
    fn largest_joltage(bank: &[u8]) -> u64 {
        let n = bank.len();
        let mut max = 0;
        for i in 0..(n-1) {
            for j in (i+1)..n {
                let jolts = (bank[i] * 10 + bank[j]) as u64;
                if jolts > max {
                    max = jolts;
                }
            }
        }
        max
    }

    let mut sum = 0;
    for bank in banks {
        sum += largest_joltage(&bank);
    }
    sum
}

fn part2(banks: &[Vec<u8>]) -> u64 {
    fn e(pow: usize) -> u64 {
        let mut ret = 1;
        for _ in 0..pow {
            ret *= 10;
        }
        ret
    }
    fn largest_joltage(bank: &[u8], i: usize, left: usize, memo: &mut HashMap<(usize, usize), u64>)
        -> Option<u64> {
        if left == 0 {
            return Some(0);
        } else if i + left > bank.len() {
            return None;
        } else if let Some(cached) = memo.get(&(i, left)) {
            return Some(*cached);
        }
        let mut max = largest_joltage(bank, i + 1, left - 1, memo).unwrap()
            + (bank[i] as u64 * e(left - 1));
        if let Some(rem) = largest_joltage(bank, i + 1, left, memo) {
            if rem > max {
                max = rem;
            }
        }
        memo.insert((i, left), max);
        Some(max)
    }

    let mut sum = 0;
    for bank in banks {
        sum += largest_joltage(&bank, 0, 12, &mut HashMap::new()).unwrap();
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let banks = parse(&puzzle_input);
    println!("{}", part1(&banks));
    println!("{}", part2(&banks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 3121910778619);
    }
}
