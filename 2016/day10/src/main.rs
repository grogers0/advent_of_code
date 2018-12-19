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

fn give(value: usize, give_to: GiveTo, bots: &mut BTreeMap<usize, Vec<usize>>,
        output: &mut BTreeMap<usize, Vec<usize>>, queue: &mut VecDeque<usize>) {
    match give_to {
        GiveTo::Bot(bot) => {
            bots.entry(bot).and_modify(|values| values.push(value)).or_insert(vec![value]);
            queue.push_back(bot);
        },
        GiveTo::Output(bin) => {
            output.entry(bin).and_modify(|values| values.push(value)).or_insert(vec![value]);
        }
    }
}

// Returns last bot compared
fn execute(input: &str, output: &mut BTreeMap<usize, Vec<usize>>, stop_comp: Option<(usize, usize)>) -> usize {
    let (comparisons, values) = parse(input);
    let mut queue = VecDeque::new();
    let mut bots = BTreeMap::<usize, Vec<usize>>::new();
    for start in values {
        queue.push_back(start.bot);
        bots.entry(start.bot).and_modify(|values| values.push(start.value)).or_insert(vec![start.value]);
    }
    let mut last_bot = 0;
    while let Some(bot) = queue.pop_front() {
        if !bots.contains_key(&bot) { continue; }
        if bots[&bot].len() <= 1 { continue; }
        assert_eq!(bots[&bot].len(), 2);

        last_bot = bot;
        let bot_vals = bots.remove(&bot).unwrap();
        let low_val = *bot_vals.iter().min().unwrap();
        give(low_val, comparisons[&bot].low, &mut bots, output, &mut queue);
        let high_val = *bot_vals.iter().max().unwrap();
        give(high_val, comparisons[&bot].high, &mut bots, output, &mut queue);

        if let Some((low_stop, high_stop)) = stop_comp {
            if low_val == low_stop && high_val == high_stop {
                break;
            }
        }
    }
    last_bot
}

fn calc_part1(input: &str, stop_comp: (usize, usize)) -> usize {
    let mut output = BTreeMap::new();
    execute(input, &mut output, Some(stop_comp))
}

fn part1(input: &str) -> usize {
    calc_part1(input, (17, 61))
}

fn part2(input: &str) -> usize {
    let mut output = BTreeMap::new();
    execute(input, &mut output, None);
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
