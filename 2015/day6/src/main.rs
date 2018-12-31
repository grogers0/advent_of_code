use std::io::{self, Read};

use regex::Regex;

type Pos = (usize, usize);

enum OpType {
    On, Off, Toggle
}

struct Op {
    op_type: OpType,
    start: Pos,
    end: Pos
}

fn parse(input: &str) -> Vec<Op> {
    let re = Regex::new("^(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)$").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let start = (cap[2].parse().unwrap(), cap[3].parse().unwrap());
            let end = (cap[4].parse().unwrap(), cap[5].parse().unwrap());
            let op_type = match &cap[1] {
                "turn on" => OpType::On,
                "turn off" => OpType::Off,
                "toggle" => OpType::Toggle,
                _ => panic!()
            };
            Op { op_type: op_type, start: start, end: end }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let mut lights = vec![false; 1_000_000];

    for Op { op_type, start: (x1,y1), end: (x2,y2) } in instructions {
        for y in y1..=y2 {
            for x in x1..=x2 {
                let light = &mut lights[y*1000 + x];
                match op_type {
                    OpType::On => { *light = true; },
                    OpType::Off => { *light = false; },
                    OpType::Toggle => { *light = !*light; }
                }
            }
        }
    }

    lights.iter().filter(|light| **light).count()
}

fn part2(input: &str) -> usize {
    let instructions = parse(input);
    let mut lights = vec![0; 1_000_000];

    for Op { op_type, start: (x1,y1), end: (x2,y2) } in instructions {
        for y in y1..=y2 {
            for x in x1..=x2 {
                let light = &mut lights[y*1000 + x];
                match op_type {
                    OpType::On => { *light += 1; },
                    OpType::Off => { if *light > 0 { *light -= 1; } },
                    OpType::Toggle => { *light += 2; }
                }
            }
        }
    }

    lights.iter().sum()
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
        assert_eq!(part1("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(part1("toggle 0,0 through 999,0"), 1000);
        assert_eq!(part1("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"), 1_000_000 - 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("turn on 0,0 through 0,0"), 1);
        assert_eq!(part2("toggle 0,0 through 999,999"), 2_000_000);
    }
}
