use std::io::{self, Read};
use regex::Regex;

fn part1(puzzle_input: &str) -> u64 {
    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(puzzle_input) {
        let x = cap[1].parse::<u64>().unwrap();
        let y = cap[2].parse::<u64>().unwrap();
        sum += x * y;
    }
    sum
}

fn part2(puzzle_input: &str) -> u64 {
    const DO: &str = "do()";
    const DONT: &str = "don't()";
    let re = Regex::new("do\\(\\)|don't\\(\\)|mul\\((\\d+),(\\d+)\\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for cap in re.captures_iter(puzzle_input) {
        if& cap[0] == DO {
            enabled = true;
        } else if &cap[0] == DONT {
            enabled = false;
        } else if enabled {
            let x = cap[1].parse::<u64>().unwrap();
            let y = cap[2].parse::<u64>().unwrap();
            sum += x * y;
        }
    }
    sum
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

    const EX1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EX2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX2), 48);
    }
}
