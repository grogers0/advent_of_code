use std::io::{self, Read};
use std::collections::HashMap;

fn parse(puzzle_input: &str) -> Vec<u64> {
    puzzle_input.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn count_digits(n: u64) -> u64 {
    let mut b = 10;
    for i in 1.. {
        if b > n { return i; }
        b *= 10;
    }
    panic!()
}

fn pow(mut b: u64, mut e: u64) -> u64 {
    let mut ret = 1;
    while e > 0 {
        if e % 2 == 1 { ret *= b; }
        b *= b;
        e /= 2;
    }
    ret
}

fn try_split(n: u64) -> Option<[u64; 2]> {
    let num_digits = count_digits(n);
    if num_digits % 2 == 1 { return None; }
    let divisor = pow(10, num_digits / 2);
    Some([n / divisor, n % divisor])
}

fn count_splits(stone: u64, blinks: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        1
    } else if let Some(ret) = memo.get(&(stone, blinks)) {
        *ret
    } else if stone == 0 {
        count_splits(1, blinks - 1, memo)
    } else if let Some([a, b]) = try_split(stone) {
        let ret = count_splits(a, blinks - 1, memo) + count_splits(b, blinks - 1, memo);
        memo.insert((stone, blinks), ret);
        ret
    } else {
        count_splits(stone * 2024, blinks - 1, memo)
    }
}

fn part1(stones: &[u64]) -> u64 {
    let mut sum = 0;
    let mut memo = HashMap::new();
    for &stone in stones {
        sum += count_splits(stone, 25, &mut memo);
    }
    sum
}

fn part2(stones: &[u64]) -> u64 {
    let mut sum = 0;
    let mut memo = HashMap::new();
    for &stone in stones {
        sum += count_splits(stone, 75, &mut memo);
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let stones = parse(&puzzle_input);
    println!("{}", part1(&stones));
    println!("{}", part2(&stones));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse("125 17")), 55312);
    }
}
