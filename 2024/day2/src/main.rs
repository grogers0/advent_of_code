use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<Vec<u64>> {
    puzzle_input.lines().map(|line| {
        line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
    }).collect()
}

fn check_decreasing(report: &[u64]) -> bool {
    report.windows(2).all(|w| w[0] <= w[1] + 3 && w[0] >= w[1] + 1)
}

fn check_increasing(report: &[u64]) -> bool {
    report.windows(2).all(|w| w[1] <= w[0] + 3 && w[1] >= w[0] + 1)
}

fn is_safe(report: &[u64]) -> bool {
    check_decreasing(report) || check_increasing(report)
}


fn part1(reports: &[Vec<u64>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn part2(reports: &[Vec<u64>]) -> usize {
    reports.iter().filter(|orig_report| {
        if is_safe(orig_report) { return true; }
        let mut report = orig_report[1..].to_vec();
        if is_safe(&report) { return true; }
        for i in 0..report.len() {
            report[i] = orig_report[i];
            if is_safe(&report) { return true; }
        }
        false
    }).count()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let reports = parse(&puzzle_input);
    println!("{}", part1(&reports));
    println!("{}", part2(&reports));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 4);
    }
}
