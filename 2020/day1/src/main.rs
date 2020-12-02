use std::collections::HashSet;
use std::io::{self, Read};

fn parse_into_hashset(puzzle_input: &str) -> HashSet<u64> {
    puzzle_input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(puzzle_input: &str) -> u64 {
    let entries = parse_into_hashset(puzzle_input);
    for v1 in entries.iter() {
        if *v1 > 2020 { continue }
        let v2 = 2020 - v1;
        if entries.contains(&v2) {
            return v1 * v2
        }
    }
    panic!()
}

fn part2(puzzle_input: &str) -> u64 {
    let entries = parse_into_hashset(puzzle_input);
    for v1 in entries.iter() {
        for v2 in entries.iter() {
            if v1 == v2 || v1 + v2 > 2020 { continue }
            let v3 = 2020 - v1 - v2;
            if entries.contains(&v3) {
                return v1 * v2 * v3;
            }
        }
    }
    panic!()
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

    const EX: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_part1() {
        assert_eq!(514579, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(241861950, part2(EX));
    }
}
