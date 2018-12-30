use std::collections::{BTreeSet, VecDeque};
use std::io::{self, Read};

use regex::Regex;

fn parse_df(input: &str) -> Vec<Vec<(u16, u16)>> {
    let re = Regex::new("^/dev/grid/node-x(\\d+)-y(\\d+)\\s+(\\d+)T\\s+(\\d+)T\\s+\\d+T\\s+\\d+%$").unwrap();
    let cap = re.captures(input.lines().rev().next().unwrap()).unwrap();
    let width = cap[1].parse::<usize>().unwrap() + 1;
    let height = cap[2].parse::<usize>().unwrap() + 1;

    let mut df = vec![vec![(0, 0); width]; height];
    for line in input.lines() {
        // NOTE - the first two lines don't match
        if let Some(cap) = re.captures(line) {
            let x: usize = cap[1].parse().unwrap();
            let y: usize = cap[2].parse().unwrap();
            df[y][x] = (cap[4].parse().unwrap(), cap[3].parse().unwrap());
        }
    }
    df
}

fn part1(input: &str) -> usize {
    let df = parse_df(input);
    let height = df.len();
    let width = df[0].len();
    let mut num_viable_pairs = 0;
    for y1 in 0..height {
        for x1 in 0..width {
            for y2 in 0..height {
                for x2 in 0..width {
                    if (x1 != x2 || y1 != y2)
                        && df[y1][x1].0 != 0
                        && df[y2][x2].1 - df[y2][x2].0 >= df[y1][x1].0
                    {
                        num_viable_pairs += 1;
                    }
                }
            }
        }
    }
    num_viable_pairs
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    // TODO - I calculated part2 by hand using the method given in the example, by moving the empty
    // square towards the goal, and then hopping the empty square around the goal, inching it
    // forwards.  Based on the reddit thread, A* is possible by detecting all the walls and using
    // the heuristic of manhattan distance from empty square to goal + 5*goal to finish.
}
