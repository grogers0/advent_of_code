use std::io::{self, Read};

fn parse(input: &str) -> Vec<usize> {
    input.trim_end().chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()
}

fn part1(input: &str) -> usize {
    let digits = parse(input);
    let mut sum = 0;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + 1) % digits.len()] {
            sum += digits[i];
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    let digits = parse(input);
    let mut sum = 0;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + digits.len()/2) % digits.len()] {
            sum += digits[i];
        }
    }
    sum
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
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1212"), 6);
        assert_eq!(part2("1221"), 0);
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
        assert_eq!(part2("12131415"), 4);
    }
}
