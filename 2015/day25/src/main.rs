use std::io::{self, Read};

use regex::Regex;

fn parse(input: &str) -> (u64, u64) {
    let re = Regex::new("^To continue, please consult the code grid in the manual.  Enter the code at row (\\d+), column (\\d+)\\.$").unwrap();
    let cap = re.captures(input.trim_end()).unwrap();
    (cap[2].parse().unwrap(), cap[1].parse().unwrap())
}

// This sequence is called the triangle numbers
fn coordinate_to_num(x: u64, y: u64) -> u64 {
    let n = x + y - 2;
    n*(n+1)/2 + x
}

fn calc(x: u64, y: u64) -> u64 {
    let num = coordinate_to_num(x, y);
    let mut code = 20151125;
    for _ in 1..num {
        code = (code * 252533) % 33554393;
    }
    code
}

fn part1(input: &str) -> u64 {
    let (x, y) = parse(input);
    calc(x, y)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(calc(4, 6), 24659492);
        assert_eq!(calc(6, 3), 16474243);
        assert_eq!(calc(1, 5), 77061);
        assert_eq!(calc(5, 1), 10071777);
    }
}
