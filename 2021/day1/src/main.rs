use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<u32> {
    puzzle_input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(measurements: &[u32]) -> usize {
    let mut cnt = 0;
    for i in 1..measurements.len() {
        if measurements[i-1] < measurements[i] {
            cnt += 1;
        }
    }
    cnt
}

fn part2(measurements: &[u32]) -> usize {
    let mut cnt = 0;
    let mut sum = measurements[0] + measurements[1] + measurements[2];
    for i in 3..measurements.len() {
        let prior_sum = sum;
        sum = sum - measurements[i - 3] + measurements[i];
        if prior_sum < sum {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let measurements = parse(&puzzle_input);
    println!("{}", part1(&measurements));
    println!("{}", part2(&measurements));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, part2(&parse(EX)));
    }
}
