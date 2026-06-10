use std::cmp::{min, max};
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::ops::RangeInclusive;

struct Inventory {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    available: Vec<usize>
}

fn parse(puzzle_input: &str) -> Inventory {
    let mut sections = puzzle_input.split("\n\n");

    let mut fresh_ranges = vec![];
    for line in sections.next().unwrap().lines() {
        let mut parts = line.split("-");
        let lower = parts.next().unwrap().parse::<usize>().unwrap();
        let upper = parts.next().unwrap().parse::<usize>().unwrap();
        assert!(parts.next().is_none());
        fresh_ranges.push(lower..=upper);
    }

    let mut available = vec![];
    for line in sections.next().unwrap().lines() {
        available.push(line.parse::<usize>().unwrap());
    }
    assert!(sections.next().is_none());

    Inventory { fresh_ranges, available }
}

fn part1(inventory: &Inventory) -> usize {
    let is_fresh = |id: usize| -> bool {
        inventory.fresh_ranges.iter().any(|range| range.contains(&id))
    };

    inventory.available.iter().filter(|&&id| is_fresh(id)).count()
}

struct IntervalSet {
    intervals: BTreeMap<usize, usize>
}

impl IntervalSet {
    fn new() -> Self { IntervalSet { intervals: BTreeMap::new() } }
    fn insert(&mut self, mut lower: usize, mut upper: usize) {
        // This is kinda goofy to re-look up the matching range but the cursor API isn't stable,
        // and it's plenty fast anyways
        loop {
            let mut matching = self.intervals.range(..=upper);
            if let Some((lower2, upper2)) = matching.next_back() {
                if lower <= *upper2 && *lower2 <= upper {
                    lower = min(lower, *lower2);
                    upper = max(upper, *upper2);
                    let lower2 = *lower2; // Hack to stop borrowing the Range iter
                    self.intervals.remove(&lower2);
                    continue;
                }
            }
            break;
        }
        self.intervals.insert(lower, upper);
    }
}

fn part2(inventory: &Inventory) -> usize {
    let mut merged = IntervalSet::new();
    for range in inventory.fresh_ranges.iter() {
        merged.insert(*range.start(), *range.end());
    }
    let mut sum = 0;
    for (lower, upper) in merged.intervals {
        sum += upper - lower + 1;
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let inventory = parse(&puzzle_input);
    println!("{}", part1(&inventory));
    println!("{}", part2(&inventory));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 14);
    }
}
