use std::io::{self, Read};
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        assert!(self.x >= other.x && self.y >= other.y);
        Pos { x: self.x - other.x, y: self.y - other.y }
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

impl Pos {
    fn scale_by(self, factor: usize) -> Pos {
        Pos { x: self.x * factor, y: self.y * factor }
    }
}

struct Machine {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}


fn parse_machine(machine_input: &str) -> Machine {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\r?\nButton B: X\+(\d+), Y\+(\d+)\r?\nPrize: X=(\d+), Y=(\d+)\r?\n?").unwrap();

    let cap = re.captures(machine_input).unwrap();
    Machine {
        button_a: Pos {
            x: cap[1].parse::<usize>().unwrap(),
            y: cap[2].parse::<usize>().unwrap(),
        },
        button_b: Pos {
            x: cap[3].parse::<usize>().unwrap(),
            y: cap[4].parse::<usize>().unwrap(),
        },
        prize: Pos {
            x: cap[5].parse::<usize>().unwrap(),
            y: cap[6].parse::<usize>().unwrap(),
        }
    }
}

fn parse(puzzle_input: &str) -> Vec<Machine> {
    let machines: Vec<Machine> = puzzle_input.split("\n\n").map(|s| parse_machine(s)).collect();
    for machine in &machines {
        // It's technically possible to solve for these cases too but the input doesn't contain
        // them so it simplifies the implementation
        assert_ne!(machine.button_a.x, machine.button_a.y);
        assert_ne!(machine.button_b.x, machine.button_b.y);
    }
    machines
}


// Ternary search... Probably a way to formulate this as binary search but it's good enough
fn cheapest_win(button_a: Pos, button_b: Pos, prize: Pos) -> Option<usize> {
    let mut left = 0;
    let mut right = 1 + std::cmp::min(prize.x / button_a.x, prize.y / button_a.y);
    let f = |a_presses: usize| -> usize {
        let pos = button_a.scale_by(a_presses);
        abs_diff((prize.x - pos.x) / button_b.x, (prize.y - pos.y) / button_b.y)
    };

    while right - left > 2 {
        let m1 = left + (right - left) / 3;
        let m2 = right - (right - left) / 3;

        if f(m1) < f(m2) {
            right = m2;
        } else {
            left = m1;

        }
    }

    for a_presses in left..right {
        let pos = button_a.scale_by(a_presses);
        if prize.x < pos.x { continue; }
        let b_presses = (prize.x - pos.x) / button_b.x;
        if pos + button_b.scale_by(b_presses) == prize {
            return Some(a_presses * 3 + b_presses);
        }
    }
    None
}

fn part1(machines: &[Machine]) -> usize {
    let mut sum = 0;
    for machine in machines {
        sum += cheapest_win(machine.button_a, machine.button_b, machine.prize).unwrap_or(0);
    }
    sum
}

fn part2(machines: &[Machine]) -> usize {
    let mut sum = 0;
    for machine in machines {
        let prize = Pos {
            x: machine.prize.x + 10000000000000, y: machine.prize.y + 10000000000000,
        };
        sum += cheapest_win(machine.button_a, machine.button_b, prize).unwrap_or(0);
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let machines = parse(&puzzle_input);
    println!("{}", part1(&machines));
    println!("{}", part2(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 480);
    }
}
