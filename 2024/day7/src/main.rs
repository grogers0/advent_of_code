use std::io::{self, Read};

struct Equation(u64, Vec<u64>);

fn parse(puzzle_input: &str) -> Vec<Equation> {
    puzzle_input.lines().map(|line| {
        let mut sp = line.split(": ");
        let result = sp.next().unwrap().parse::<u64>().unwrap();
        let values = sp.next().unwrap().split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
        assert!(sp.next().is_none());
        Equation(result, values)
    }).collect()
}

fn can_satisfy(equation: &Equation, is_part2: bool) -> bool {
    fn concat(a: u64, b: u64) -> u64 {
        let mut n = 10;
        while n <= b {
            n *= 10;
        }
        a * n + b
    }
    fn can_satisfy_recur(expected: u64, curr: u64, remaining: &[u64], is_part2: bool) -> bool {
        if remaining.is_empty() { return expected == curr; }
        let next = remaining[0];
        let remaining = &remaining[1..];
        if can_satisfy_recur(expected, curr * next, remaining, is_part2) { return true; }
        if can_satisfy_recur(expected, curr + next, remaining, is_part2) { return true; }
        is_part2 && can_satisfy_recur(expected, concat(curr, next), remaining, is_part2)
    }

    let curr = equation.1[0];
    let remaining = &equation.1[1..];
    can_satisfy_recur(equation.0, curr, remaining, is_part2)
}

fn part1(equations: &[Equation]) -> u64 {
    let mut sum = 0;
    for equation in equations {
        if can_satisfy(equation, false) {
            sum += equation.0;
        }
    }
    sum
}

fn part2(equations: &[Equation]) -> u64 {
    let mut sum = 0;
    for equation in equations {
        if can_satisfy(equation, true) {
            sum += equation.0;
        }
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let equations = parse(&puzzle_input);
    println!("{}", part1(&equations));
    println!("{}", part2(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 11387);
    }
}
