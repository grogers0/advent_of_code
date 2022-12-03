use std::io::{self, Read};

fn priority(ch: char) -> u64 {
    match ch {
        'a'..='z' => (ch as u64 - 'a' as u64) + 1,
        'A'..='Z' => (ch as u64 - 'A' as u64) + 27,
        _ => panic!(),
    }
}

fn part1(puzzle_input: &str) -> u32 {
    puzzle_input.trim_end().split("\n").map(|line| {
        let compartment_len = line.chars().count() / 2;
        let mut left = 0u64;
        let mut right = 0u64;
        for (i, ch) in line.chars().enumerate() {
            if i < compartment_len {
                left |= 1 << priority(ch);
            } else {
                right |= 1 << priority(ch);
            }
        }
        let overlap = left & right;
        assert_eq!(overlap.count_ones(), 1);
        overlap.trailing_zeros()
    }).sum()
}

fn part2(puzzle_input: &str) -> u32 {
    let rucksacks = puzzle_input.trim_end().split("\n").map(|line| {
        let mut sack = 0u64;
        for ch in line.chars() {
            sack |= 1 << priority(ch);
        }
        sack
    }).collect::<Vec<_>>();
    rucksacks.chunks(3).map(|group| {
        let overlap = group[0] & group[1] & group[2];
        assert_eq!(overlap.count_ones(), 1);
        overlap.trailing_zeros()
    }).sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 70);
    }
}
