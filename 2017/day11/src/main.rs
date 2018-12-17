use std::collections::BTreeMap;
use std::convert::From;
use std::cmp::min;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    NW, N, NE,
    SW, S, SE
}

impl From<&str> for Dir {
    fn from(s: &str) -> Dir {
        match s {
            "nw" => Dir::NW,
            "n"  => Dir::N,
            "ne" => Dir::NE,
            "sw" => Dir::SW,
            "s"  => Dir::S,
            "se" => Dir::SE,
            _    => unreachable!()

        }
    }
}

fn parse(input: &str) -> Vec<Dir> {
    input.trim_end().split(",")
        .map(|dir| Dir::from(dir))
        .collect()
}

// Any steps can be transposed and will reach the same destination, so just having the counts of
// each direction completely represents the end
fn dir_counts(steps: &[Dir]) -> BTreeMap<Dir, usize> {
    let mut counts: BTreeMap<Dir, usize> = BTreeMap::new();
    counts.insert(Dir::NW, 0);
    counts.insert(Dir::N, 0);
    counts.insert(Dir::NE, 0);
    counts.insert(Dir::SW, 0);
    counts.insert(Dir::S, 0);
    counts.insert(Dir::SE, 0);
    for step in steps {
        counts.entry(*step).and_modify(|cnt| *cnt += 1);
    }
    counts
}

fn distance(counts: &BTreeMap<Dir, usize>) -> usize {
    counts.values().sum()
}

// Returns whether any were cancelled
fn cancel_opposites(counts: &mut BTreeMap<Dir, usize>) -> bool {
    let orig_len = distance(counts);

    if counts[&Dir::N] > counts[&Dir::S] {
        let num = counts[&Dir::N] - counts[&Dir::S];
        counts.insert(Dir::N, num);
        counts.insert(Dir::S, 0);
    } else {
        let num = counts[&Dir::S] - counts[&Dir::N];
        counts.insert(Dir::S, num);
        counts.insert(Dir::N, 0);
    }

    if counts[&Dir::NW] > counts[&Dir::SE] {
        let num = counts[&Dir::NW] - counts[&Dir::SE];
        counts.insert(Dir::NW, num);
        counts.insert(Dir::SE, 0);
    } else {
        let num = counts[&Dir::SE] - counts[&Dir::NW];
        counts.insert(Dir::SE, num);
        counts.insert(Dir::NW, 0);
    }

    if counts[&Dir::NE] > counts[&Dir::SW] {
        let num = counts[&Dir::NE] - counts[&Dir::SW];
        counts.insert(Dir::NE, num);
        counts.insert(Dir::SW, 0);
    } else {
        let num = counts[&Dir::SW] - counts[&Dir::NE];
        counts.insert(Dir::SW, num);
        counts.insert(Dir::NE, 0);
    }

    orig_len != distance(counts)
}

// Returns whether any were cancelled
fn cancel_one_step_off(counts: &mut BTreeMap<Dir, usize>) -> bool {
    let orig_len = distance(counts);
    {
        let num = min(counts[&Dir::NW], counts[&Dir::NE]);
        counts.entry(Dir::N).and_modify(|cnt| *cnt += num);
        counts.entry(Dir::NW).and_modify(|cnt| *cnt -= num);
        counts.entry(Dir::NE).and_modify(|cnt| *cnt -= num);
    }
    {
        let num = min(counts[&Dir::N], counts[&Dir::SE]);
        counts.entry(Dir::NE).and_modify(|cnt| *cnt += num);
        counts.entry(Dir::N).and_modify(|cnt| *cnt -= num);
        counts.entry(Dir::SE).and_modify(|cnt| *cnt -= num);
    }
    {
        let num = min(counts[&Dir::NE], counts[&Dir::S]);
        counts.entry(Dir::SE).and_modify(|cnt| *cnt += num);
        counts.entry(Dir::NE).and_modify(|cnt| *cnt -= num);
        counts.entry(Dir::S).and_modify(|cnt| *cnt -= num);
    }
    {
        let num = min(counts[&Dir::SW], counts[&Dir::SE]);
        counts.entry(Dir::S).and_modify(|cnt| *cnt += num);
        counts.entry(Dir::SW).and_modify(|cnt| *cnt -= num);
        counts.entry(Dir::SE).and_modify(|cnt| *cnt -= num);
    }
    {
        let num = min(counts[&Dir::S], counts[&Dir::NW]);
        counts.entry(Dir::SW).and_modify(|cnt| *cnt += num);
        counts.entry(Dir::S).and_modify(|cnt| *cnt -= num);
        counts.entry(Dir::NW).and_modify(|cnt| *cnt -= num);
    }
    {
        let num = min(counts[&Dir::SW], counts[&Dir::N]);
        counts.entry(Dir::NW).and_modify(|cnt| *cnt += num);
        counts.entry(Dir::SW).and_modify(|cnt| *cnt -= num);
        counts.entry(Dir::N).and_modify(|cnt| *cnt -= num);
    }

    orig_len != distance(counts)
}

fn distance_from_start(steps: &[Dir]) -> usize {
    let mut counts = dir_counts(steps);
    loop {
        cancel_opposites(&mut counts);
        if cancel_one_step_off(&mut counts) { continue }
        break;
    }
    distance(&counts)
}

fn part1(input: &str) -> usize {
    let steps = parse(input);
    distance_from_start(&steps)
}

fn part2(input: &str) -> usize {
    let steps = parse(input);
    (0..steps.len()).map(|i| {
        distance_from_start(&steps[0..i])
    })
    .max().unwrap()
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

    #[test]
    fn test_part1() {
        assert_eq!(part1("ne,ne,ne"), 3);
        assert_eq!(part1("ne,ne,sw,sw"), 0);
        assert_eq!(part1("ne,ne,s,s"), 2);
        assert_eq!(part1("se,sw,se,sw,sw"), 3);
    }
}
