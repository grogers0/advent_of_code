use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum GiveTo {
    Bot(usize),
    Output(usize)
}

struct Compare {
    low: GiveTo,
    high: GiveTo
}

struct Start {
    value: usize,
    bot: usize
}

fn parse(input: &str) -> (BTreeMap<usize, Compare>, Vec<Start>) {
    lazy_static!{
        static ref VALUE_RE: Regex = Regex::new("^value (\\d+) goes to bot (\\d+)$").unwrap();
        static ref COMPARE_RE: Regex = Regex::new("^bot (\\d+) gives low to (bot|output) (\\d+) and high to (bot|output) (\\d+)$").unwrap();
    }

    let mut values = Vec::new();
    let mut comparisons = BTreeMap::new();
    for line in input.lines() {
        if let Some(cap) = VALUE_RE.captures(line) {
            values.push(Start { value: cap[1].parse().unwrap(), bot: cap[2].parse().unwrap() });
        } else if let Some(cap) = COMPARE_RE.captures(line) {
            let parse_giveto = |bot_str: &str, num_str: &str| -> GiveTo {
                let num = num_str.parse().unwrap();
                match bot_str {
                    "bot" => GiveTo::Bot(num),
                    "output" => GiveTo::Output(num),
                    _ => unreachable!()
                }
            };
            comparisons.insert(cap[1].parse().unwrap(),
                Compare { low: parse_giveto(&cap[2], &cap[3]), high: parse_giveto(&cap[4], &cap[5]) });
        } else {
            unreachable!()
        }
    }
    (comparisons, values)
}

fn calc_part1(input: &str, terminal_values: (usize, usize)) -> usize {
    let (comparisons, values) = parse(input);
    let mut queue = VecDeque::new();
    let mut output = BTreeMap::<usize, Vec<usize>>::new();
    let mut bots = BTreeMap::<usize, Vec<usize>>::new();
    for start in values {
        queue.push_back(start.bot);
        bots.entry(start.bot).and_modify(|values| values.push(start.value)).or_insert(vec![start.value]);
    }
    while let Some(bot) = queue.pop_front() {
        if !bots.contains_key(&bot) { continue; }
        if bots[&bot].len() <= 1 { continue; }
        assert_eq!(bots[&bot].len(), 2);

        let low = *bots[&bot].iter().min().unwrap();
        let high = *bots[&bot].iter().max().unwrap();
        if low == terminal_values.0 && high == terminal_values.1 {
            return bot
        }
        bots.remove(&bot);
        match comparisons[&bot].low {
            GiveTo::Bot(lowbot) => {
                bots.entry(lowbot).and_modify(|values| values.push(low)).or_insert(vec![low]);
                queue.push_back(lowbot);
            },
            GiveTo::Output(bin) => { output.entry(bin).and_modify(|values| values.push(low)).or_insert(vec![low]); }
        }
        match comparisons[&bot].high {
            GiveTo::Bot(highbot) => {
                bots.entry(highbot).and_modify(|values| values.push(high)).or_insert(vec![high]);
                queue.push_back(highbot);
            },
            GiveTo::Output(bin) => { output.entry(bin).and_modify(|values| values.push(high)).or_insert(vec![high]); }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    calc_part1(input, (17, 61))
}

// FIXME - really hacky copy/paste but too lazy to fix now
fn part2(input: &str) -> usize {
    let (comparisons, values) = parse(input);
    let mut queue = VecDeque::new();
    let mut output = BTreeMap::<usize, Vec<usize>>::new();
    let mut bots = BTreeMap::<usize, Vec<usize>>::new();
    for start in values {
        queue.push_back(start.bot);
        bots.entry(start.bot).and_modify(|values| values.push(start.value)).or_insert(vec![start.value]);
    }
    while let Some(bot) = queue.pop_front() {
        if !bots.contains_key(&bot) { continue; }
        if bots[&bot].len() <= 1 { continue; }
        assert_eq!(bots[&bot].len(), 2);

        let low = *bots[&bot].iter().min().unwrap();
        let high = *bots[&bot].iter().max().unwrap();
        bots.remove(&bot);
        match comparisons[&bot].low {
            GiveTo::Bot(lowbot) => {
                bots.entry(lowbot).and_modify(|values| values.push(low)).or_insert(vec![low]);
                queue.push_back(lowbot);
            },
            GiveTo::Output(bin) => { output.entry(bin).and_modify(|values| values.push(low)).or_insert(vec![low]); }
        }
        match comparisons[&bot].high {
            GiveTo::Bot(highbot) => {
                bots.entry(highbot).and_modify(|values| values.push(high)).or_insert(vec![high]);
                queue.push_back(highbot);
            },
            GiveTo::Output(bin) => { output.entry(bin).and_modify(|values| values.push(high)).or_insert(vec![high]); }
        }
    }
    output[&0][0] * output[&1][0] * output[&2][0]
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
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn test_part1() {
        assert_eq!(calc_part1(EX, (2, 5)), 2);
    }
}
