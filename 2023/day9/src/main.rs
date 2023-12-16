use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<Vec<i64>> {
    puzzle_input.lines().map(|line| {
        line.split_whitespace().map(|token| {
            token.parse().unwrap()
        }).collect()
    }).collect()
}

fn extrapolate_forward(sequence: &[i64]) -> i64 {
    let mut differences = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        differences.push(sequence[i] - sequence[i-1]);
    }
    let next_difference = if differences.iter().all(|&d| d == 0) {
        0
    } else {
        extrapolate_forward(&differences)
    };
    sequence[sequence.len() - 1] + next_difference
}

fn extrapolate_backward(sequence: &[i64]) -> i64 {
    let mut differences = Vec::with_capacity(sequence.len() - 1);
    for i in 1..sequence.len() {
        differences.push(sequence[i] - sequence[i-1]);
    }
    let prev_difference = if differences.iter().all(|&d| d == 0) {
        0
    } else {
        extrapolate_backward(&differences)
    };
    sequence[0] - prev_difference
}

fn part1(sequences: &[Vec<i64>]) -> i64 {
    sequences.iter().map(|seq| extrapolate_forward(seq)).sum()
}

fn part2(sequences: &[Vec<i64>]) -> i64 {
    sequences.iter().map(|seq| extrapolate_backward(seq)).sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let sequences = parse(&puzzle_input);
    println!("{}", part1(&sequences));
    println!("{}", part2(&sequences));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 2);
    }
}
