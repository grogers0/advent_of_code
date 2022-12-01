use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<u64> {
    puzzle_input.trim_end().split("\n\n")
        .map(|cals_input| {
            cals_input.split("\n")
                .map(|line| line.parse::<u64>().unwrap())
                .sum()
        })
        .collect()
}


fn part1(elves: &[u64]) -> u64 {
    elves.iter().copied().max().unwrap()
}

fn part2(elves: &[u64]) -> u64 {
    let mut heap = BinaryHeap::new();
    for &cals in elves {
        heap.push(Reverse(cals));
        if heap.len() > 3 { heap.pop(); }
    }
    heap.iter().map(|Reverse(cals)| cals).sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let elves = parse(&puzzle_input);
    println!("{}", part1(&elves));
    println!("{}", part2(&elves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_parse() {
        assert_eq!(parse(EX), vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 45000);
    }
}
