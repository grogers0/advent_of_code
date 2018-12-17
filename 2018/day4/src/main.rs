use std::collections::BTreeMap;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn matches_with_extraction(re: &Regex, input: &str) -> Option<usize> {
    if let Some(cap) = re.captures(input) {
        Some(cap[1].parse().unwrap())
    } else {
        None
    }
}

// Guard ID to total number of asleep minutes by the minute number
fn get_asleep_minutes(input: &str) -> BTreeMap<usize, [usize; 60]> {
    let mut input_lines: Vec<&str> = input.lines().collect();
    input_lines.sort_unstable();

    lazy_static! {
        static ref SHIFT_CHANGE_RE: Regex =
            Regex::new("^\\[\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}\\] Guard #(\\d+) begins shift$").unwrap();
        static ref FALLS_ASLEEP_RE: Regex =
            Regex::new("^\\[\\d{4}-\\d{2}-\\d{2} \\d{2}:(\\d{2})\\] falls asleep$").unwrap();
        static ref WAKES_UP_RE: Regex =
            Regex::new("^\\[\\d{4}-\\d{2}-\\d{2} \\d{2}:(\\d{2})\\] wakes up$").unwrap();
    }

    let mut curr_guard = 0;
    let mut is_asleep = false;
    let mut last_minute = 0;
    let mut asleep_minutes = BTreeMap::<usize, [usize; 60]>::new();

    for line in input_lines {
        if let Some(next_guard) = matches_with_extraction(&SHIFT_CHANGE_RE, line) {
            if is_asleep { // last guard was asleep to the end of their shift
                for minute in last_minute .. 60 {
                    asleep_minutes.get_mut(&curr_guard).unwrap()[minute] += 1;
                }
            }

            asleep_minutes.entry(next_guard).or_insert([0; 60]);
            curr_guard = next_guard;
            is_asleep = false;
            last_minute = 0;
        } else if let Some(next_minute) = matches_with_extraction(&FALLS_ASLEEP_RE, line) {
            assert!(!is_asleep);
            is_asleep = true;
            last_minute = next_minute;
        } else if let Some(next_minute) = matches_with_extraction(&WAKES_UP_RE, line) {
            assert!(is_asleep);
            for minute in last_minute .. next_minute {
                asleep_minutes.get_mut(&curr_guard).unwrap()[minute] += 1;
            }

            is_asleep = false;
            last_minute = next_minute;
        } else {
            unreachable!();
        }
    }

    asleep_minutes
}

fn part1(input: &str) -> usize {
    let asleep_minutes = get_asleep_minutes(input);
    let (best_guard, guard_minutes) = asleep_minutes.iter().max_by_key(|(_,counts)| counts.iter().sum::<usize>()).unwrap();
    let best_minute = guard_minutes.iter().enumerate().max_by_key(|(_,count)| *count).unwrap().0;
    best_guard * best_minute
}

fn part2(input: &str) -> usize {
    let asleep_minutes = get_asleep_minutes(input);
    let (best_guard, guard_minutes) = asleep_minutes.iter().max_by_key(|(_,counts)| counts.iter().max()).unwrap();
    let best_minute = guard_minutes.iter().enumerate().max_by_key(|(_,count)| *count).unwrap().0;
    best_guard * best_minute
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
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 4455);
    }

}
