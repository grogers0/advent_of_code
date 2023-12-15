use std::io::{self, Read};

struct Race {
    time: u64,
    dist: u64,
}

fn parse(puzzle_input: &str) -> Vec<Race> {
    let mut lines = puzzle_input.lines();
    let time_line = lines.next().unwrap();
    let dist_line = lines.next().unwrap();
    assert!(lines.next().is_none());
    const TIME_STR: &str = "Time:";
    assert!(time_line.starts_with(TIME_STR));
    let time_line = &time_line[TIME_STR.len()..];

    const DIST_STR: &str = "Distance:";
    assert!(dist_line.starts_with(DIST_STR));
    let dist_line = &dist_line[DIST_STR.len()..];

    let mut time_iter = time_line.split_whitespace();
    let mut dist_iter = dist_line.split_whitespace();

    let mut races = Vec::new();
    while let Some(time_str) = time_iter.next() {
        let dist_str = dist_iter.next().unwrap();
        let time = time_str.parse().unwrap();
        let dist = dist_str.parse().unwrap();
        races.push(Race { time, dist });
    }
    races
}

// TODO - Could definitely be faster. There's probably an analytical solution, but even binary
// search would be way faster. Still, this is fast enough.
fn ways_to_win(race: &Race) -> u64 {
    let mut cnt = 0;
    for hold in 1..race.time {
        if hold * (race.time - hold) > race.dist {
            cnt += 1;
        }
    }
    cnt
}

fn part1(races: &[Race]) -> u64 {
    races.iter().map(|race| ways_to_win(race)).product()
}

fn combine_races(races: &[Race]) -> Race {
    fn round_up_pow10(n: u64) -> u64 {
        let mut p = 1;
        while p < n {
            p *= 10;
        }
        p
    }
    let mut time = 0;
    let mut dist = 0;
    for race in races {
        time = time * round_up_pow10(race.time) + race.time;
        dist = dist * round_up_pow10(race.dist) + race.dist;
    }
    Race { time, dist }
}

fn part2(races: &[Race]) -> u64 {
    let race = combine_races(races);
    ways_to_win(&race)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let races = parse(&puzzle_input);
    println!("{}", part1(&races));
    println!("{}", part2(&races));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 71503);
    }
}
