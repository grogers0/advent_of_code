use std::io::{self, Read};
use std::ops::RangeInclusive;

const NON_DIGIT: u8 = std::u8::MAX;

fn parse(input: &str) -> RangeInclusive<u32> {
    let mut sp = input.trim().split("-");
    let start = sp.next().unwrap().parse().unwrap();
    let end = sp.next().unwrap().parse().unwrap();
    start ..= end
}

fn digits(mut num: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }
    digits.reverse();
    digits
}

fn has_adjacent_matching(digits: &[u8]) -> bool {
    let mut last_digit = NON_DIGIT;
    for digit in digits {
        if last_digit == *digit { return true; }
        last_digit = *digit;
    }
    false
}

fn has_adjacent_matching_strictly(digits: &[u8]) -> bool {
    let mut last_digit = NON_DIGIT;
    let mut group_size = 0;
    for digit in digits {
        if last_digit == *digit {
            group_size += 1;
        } else if group_size == 2 {
            return true;
        } else {
            last_digit = *digit;
            group_size = 1;
        }
    }
    group_size == 2
}

fn monotonically_increasing(digits: &[u8]) -> bool {
    let mut last_digit = 0;
    for digit in digits {
        if last_digit > *digit { return false; }
        last_digit = *digit;
    }
    true
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().filter(|num| {
        let digits = digits(*num);
        has_adjacent_matching(&digits) && monotonically_increasing(&digits)
    }).count()
}

fn part2(input: &str) -> usize {
    parse(input).into_iter().filter(|num| {
        let digits = digits(*num);
        has_adjacent_matching_strictly(&digits) && monotonically_increasing(&digits)
    }).count()
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
    fn test_part1() {
        assert!(has_adjacent_matching(&digits(111111)));
        assert!(has_adjacent_matching(&digits(223450)));
        assert!(!has_adjacent_matching(&digits(123789)));

        assert!(monotonically_increasing(&digits(111111)));
        assert!(!monotonically_increasing(&digits(223450)));
        assert!(monotonically_increasing(&digits(123789)));
    }

    #[test]
    fn test_part2() {
        assert!(has_adjacent_matching_strictly(&digits(112233)));
        assert!(!has_adjacent_matching_strictly(&digits(123444)));
        assert!(has_adjacent_matching_strictly(&digits(111122)));
    }
}
