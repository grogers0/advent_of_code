use std::io::{self, Read};

use regex::Regex;

fn parse(puzzle_input: &str) -> Vec<String> {
    puzzle_input.lines().map(|s| s.to_string()).collect()
}

fn part1(lines: &[String]) -> u32 {
    let mut sum = 0;
    for line in lines {
        let mut seen_first = false;
        let mut first = 0;
        let mut last = 0;
        for ch in line.chars() {
            if ch < '0' || ch > '9' {
                continue;
            }
            let num = (ch as u32) - '0' as u32;
            if !seen_first {
                first = num;
                seen_first = true;
            }
            last = num;
        }
        sum += first * 10 + last;
    }
    sum
}

fn str_to_num(s: &str) -> u32 {
    match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!()
    }
}

fn part2(lines: &[String]) -> u32 {
    let first_regex = Regex::new(".*?([0-9]|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let last_regex = Regex::new(".*([0-9]|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let mut sum = 0;
    for line in lines {
        let first_cap = first_regex.captures(&line).unwrap();
        let last_cap = last_regex.captures(&line).unwrap();
        sum += str_to_num(&first_cap[1]) * 10 + str_to_num(&last_cap[1]);
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let lines = parse(&puzzle_input);
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 142);
    }

    const EX2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX2)), 281);
    }
}
