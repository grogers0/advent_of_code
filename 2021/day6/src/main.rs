use std::io::{self, Read};

const MAX_DAYS: usize = 9;

fn parse(puzzle_input: &str) -> [u64; MAX_DAYS] {
    let mut fish_timers = [0; MAX_DAYS];
    for s in puzzle_input.trim_end().split(",") {
        fish_timers[s.parse::<usize>().unwrap()] += 1;
    }
    fish_timers
}

fn count_fish_after_days(fish_timers: &[u64; MAX_DAYS], days: usize) -> u64 {
    let mut fish_timers = fish_timers.clone();
    for _ in 0..days {
        let fish_0 = fish_timers[0];
        for i in 0..MAX_DAYS-1 {
            fish_timers[i] = fish_timers[i + 1];
        }
        fish_timers[8] = fish_0;
        fish_timers[6] += fish_0;
    }

    fish_timers.iter().sum()
}

fn part1(fish_timers: &[u64; MAX_DAYS]) -> u64 {
    count_fish_after_days(fish_timers, 80)
}

fn part2(fish_timers: &[u64; MAX_DAYS]) -> u64 {
    count_fish_after_days(fish_timers, 256)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let fish_timers = parse(&puzzle_input);
    println!("{}", part1(&fish_timers));
    println!("{}", part2(&fish_timers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(5934, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539, part2(&parse(EX)));
    }
}

