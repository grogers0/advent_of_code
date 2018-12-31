use std::collections::BTreeSet;
use std::io::{self, Read};

fn take_step(ch: char, pos: &mut (i64, i64)) {
    match ch {
        '^' => pos.1 -= 1,
        'v' => pos.1 += 1,
        '<' => pos.0 -= 1,
        '>' => pos.0 += 1,
        _ => panic!()
    }
}

fn part1(input: &str) -> usize {
    let mut seen = BTreeSet::new();
    let mut pos = (0, 0);
    seen.insert(pos);
    for ch in input.trim_end().chars() {
        take_step(ch, &mut pos);
        seen.insert(pos);
    }
    seen.len()
}

fn part2(input: &str) -> usize {
    let mut seen = BTreeSet::new();
    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    seen.insert(santa_pos);
    let mut real_santa = true;
    for ch in input.trim_end().chars() {
        if real_santa {
            take_step(ch, &mut santa_pos);
            seen.insert(santa_pos);
        } else {
            take_step(ch, &mut robo_pos);
            seen.insert(robo_pos);
        }
        real_santa = !real_santa;
    }
    seen.len()
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
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
