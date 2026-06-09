use std::io::{self, Read};

struct Rotation {
    inc: bool,
    dist: u64,
}

fn parse(puzzle_input: &str) -> Vec<Rotation> {
    let mut ret = vec![];
    for line in puzzle_input.lines() {
        let inc = match &line[0..=0] {
            "L" => false,
            "R" => true,
            _ => panic!(),
        };
        let dist = line[1..].parse::<u64>().unwrap();
        ret.push(Rotation { inc, dist });
    }
    ret
}

fn part1(rotations: &[Rotation]) -> u64 {
    let mut dial = 50;
    let mut cnt = 0;
    for rot in rotations {
        dial = if rot.inc {
            (dial + (rot.dist % 100)) % 100
        } else {
            (dial + 100 - (rot.dist % 100)) % 100
        };

        if dial == 0 {
            cnt += 1;
        }
    }
    cnt
}

fn part2(rotations: &[Rotation]) -> u64 {
    let mut dial = 50;
    let mut cnt = 0;
    for rot in rotations {
        let old_dial = dial;
        dial = if rot.inc {
            (dial + (rot.dist % 100)) % 100
        } else {
            (dial + 100 - (rot.dist % 100)) % 100
        };

        cnt += rot.dist / 100;
        if rot.inc {
            if dial < old_dial { cnt += 1 };
        } else {
            if old_dial != 0 && (dial == 0 || dial > old_dial) { cnt += 1 };
        }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let rotations = parse(&puzzle_input);
    println!("{}", part1(&rotations));
    println!("{}", part2(&rotations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 6);
    }
}
