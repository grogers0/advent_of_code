use std::io::{self, Read};
use std::ops::RangeInclusive;

fn parse(puzzle_input: &str) -> Vec<[RangeInclusive<u32>; 2]> {
    let parse_range = |range: &str| -> RangeInclusive<u32> {
        let mut sp = range.split("-");
        let start = sp.next().unwrap().parse().unwrap();
        let end = sp.next().unwrap().parse().unwrap();
        assert!(sp.next().is_none());
        start ..= end
    };
    puzzle_input.trim_end().split("\n").map(|line| {
        let mut sp = line.split(",");
        let first = parse_range(sp.next().unwrap());
        let second = parse_range(sp.next().unwrap());
        assert!(sp.next().is_none());
        [first, second]
    }).collect()
}

fn fully_contains(outer: &RangeInclusive<u32>, inner: &RangeInclusive<u32>) -> bool {
    outer.start() <= inner.start() && outer.end() >= inner.end()
}

fn contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.end() >= b.start() && b.end() >= a.start() 
}

fn part1(elf_pairs: &[[RangeInclusive<u32>; 2]]) -> usize {
    elf_pairs.iter().filter(|[a, b]| fully_contains(a, b) || fully_contains(b, a)).count()
}

fn part2(elf_pairs: &[[RangeInclusive<u32>; 2]]) -> usize {
    elf_pairs.iter().filter(|[a, b]| contains(a, b)).count()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let elf_pairs = parse(&puzzle_input);
    println!("{}", part1(&elf_pairs));
    println!("{}", part2(&elf_pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 4);
    }
}
