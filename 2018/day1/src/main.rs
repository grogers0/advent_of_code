use std::collections::BTreeSet;
use std::io::{self, Read};

fn part1(input: &str) -> i32 {
    input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut seen = BTreeSet::new();
    let mut curr = 0;
    loop {
        for incr in input.lines().map(|line| line.parse::<i32>().unwrap()) {
            seen.insert(curr);
            curr += incr;
            if seen.contains(&curr) {
                return curr;
            }
        }
    }
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
        assert_eq!(3, part1("+1\n+1\n+1"));
        assert_eq!(0, part1("+1\n+1\n-2"));
        assert_eq!(-6, part1("-1\n-2\n-3"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2("+1\n-1"));
        assert_eq!(10, part2("+3\n+3\n+4\n-2\n-4"));
        assert_eq!(5, part2("-6\n+3\n+8\n+5\n-6"));
        assert_eq!(14, part2("+7\n+7\n-2\n-7\n-4"));
    }
}
