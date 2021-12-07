use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<u32> {
    puzzle_input.trim_end().split(",").map(|s| s.parse().unwrap()).collect()
}

fn abs_dist(a: u32, b: u32) -> u32 {
    if a <= b {
        b - a
    } else {
        a - b
    }
}

fn best_cost(positions: &[u32], cost_fn: fn(u32, u32) -> u32) -> u32 {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    (min_pos .. max_pos+1)
        .map(|dest_pos| positions.iter()
            .map(|&pos| cost_fn(dest_pos, pos))
            .sum())
        .min()
        .unwrap()
}

fn part1(positions: &[u32]) -> u32 {
    best_cost(positions, abs_dist)
}

fn part2(positions: &[u32]) -> u32 {
    let binomial_dist = |a, b| abs_dist(a, b) * (abs_dist(a, b) + 1) / 2;
    best_cost(positions, binomial_dist)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let positions = parse(&puzzle_input);
    println!("{}", part1(&positions));
    println!("{}", part2(&positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(37, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, part2(&parse(EX)));
    }
}
