use std::io::{self, Read};

struct Cmd(CmdType, u32);

enum CmdType {
    Forward, Down, Up
}

fn parse(puzzle_input: &str) -> Vec<Cmd> {
    const FORWARD: &str = "forward ";
    const DOWN: &str = "down ";
    const UP: &str = "up ";
    puzzle_input.lines().map(|line| {
        if line.starts_with(FORWARD) {
            Cmd(CmdType::Forward, line[FORWARD.len()..].parse().unwrap())
        } else if line.starts_with(DOWN) {
            Cmd(CmdType::Down, line[DOWN.len()..].parse().unwrap())
        } else if line.starts_with(UP) {
            Cmd(CmdType::Up, line[UP.len()..].parse().unwrap())
        } else {
            panic!()
        }
    }).collect()
}

fn part1(commands: &[Cmd]) -> u32 {
    let mut depth = 0;
    let mut pos = 0;
    for Cmd(typ, amt) in commands {
        match typ {
            CmdType::Forward => pos += amt,
            CmdType::Down => depth += amt,
            CmdType::Up => depth -= amt
        }
    }
    depth * pos
}

fn part2(commands: &[Cmd]) -> u32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for Cmd(typ, amt) in commands {
        match typ {
            CmdType::Forward => {
                pos += amt;
                depth += amt * aim;
            },
            CmdType::Down => aim += amt,
            CmdType::Up => aim -= amt
        }
    }
    depth * pos
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let commands = parse(&puzzle_input);
    println!("{}", part1(&commands));
    println!("{}", part2(&commands));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        assert_eq!(150, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(900, part2(&parse(EX)));
    }
}
