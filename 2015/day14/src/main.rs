use std::collections::BTreeMap;
use std::cmp::min;
use std::io::{self, Read};

use regex::Regex;

struct Stats {
    speed: u64,
    flight_time: u64,
    rest_time: u64
}

fn parse(input: &str) -> BTreeMap<String, Stats> {
    let re = Regex::new("^([A-Za-z]+) can fly (\\d+) km/s for (\\d+) seconds, but then must rest for (\\d+) seconds.$").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (cap[1].to_string(),
            Stats {
                speed: cap[2].parse().unwrap(),
                flight_time: cap[3].parse().unwrap(),
                rest_time: cap[4].parse().unwrap()
            })
        })
        .collect()
}

fn distance_traveled(stats: &Stats, duration: u64) -> u64 {
    let mut time = 0;
    let mut dist = 0;
    while time < duration {
        dist += stats.speed * min(stats.flight_time, duration - time);
        time += stats.flight_time + stats.rest_time;
    }
    dist
}

fn race1(reindeer: &BTreeMap<String, Stats>, duration: u64) -> u64 {
    reindeer.values().map(|stats| distance_traveled(stats, duration)).max().unwrap()
}

fn race2(reindeer: &BTreeMap<String, Stats>, duration: u64) -> u64 {
    fn leaders(distances: &BTreeMap<String, u64>) -> Vec<String> {
        let max_dist = *distances.values().max().unwrap();
        distances.iter().filter(|(_,d)| **d == max_dist).map(|(r,_)| r.to_string()).collect()
    }
    let mut points: BTreeMap<_,_> = reindeer.keys().map(|r| (r.clone(), 0)).collect();
    for time in 1..=duration {
        // NOTE - Recalculating this each time is plenty fast enough
        let distances = reindeer.iter().map(|(r, stats)| (r.to_string(), distance_traveled(stats, time))).collect();
        for leader in leaders(&distances) {
            points.entry(leader).and_modify(|cnt| *cnt += 1);
        }
    }
    *points.values().max().unwrap()
}

fn part1(input: &str) -> u64 {
    race1(&parse(input), 2503)
}

fn part2(input: &str) -> u64 {
    race2(&parse(input), 2503)
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

    const EX: &str = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_part1() {
        assert_eq!(race1(&parse(EX), 1000), 1120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(race2(&parse(EX), 1000), 689);
    }
}
