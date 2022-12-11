use std::io::{self, Read};

enum Op {
    Noop,
    Addx(i64),
}

impl Op {
    fn cycles(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

fn parse(puzzle_input: &str) -> Vec<Op> {
    const NOOP: &str = "noop";
    const ADDX_PREFIX: &str = "addx ";
    puzzle_input.trim_end().lines().map(|line| {
        if line.starts_with(ADDX_PREFIX) {
            Op::Addx(line[ADDX_PREFIX.len()..].parse().unwrap())
        } else if line == NOOP {
            Op::Noop
        } else {
            panic!()
        }
    }).collect()
}

struct Cpu<'a> {
    program: &'a Vec<Op>,
    pc: usize,
    x: i64,
    cycles_to_retire: usize,
}

impl <'a> Cpu<'a> {
    fn new(program: &'a Vec<Op>) -> Cpu<'a> {
        Cpu { program, pc: 0, x: 1, cycles_to_retire: 0 }
    }

    // Returns the value of x 
    fn step(&mut self) -> Option<i64> {
        if self.pc >= self.program.len() {
            return None;
        }
        if self.cycles_to_retire == 0 {
            self.cycles_to_retire = self.program[self.pc].cycles();
        }
        let ret = Some(self.x);
        self.cycles_to_retire -= 1;
        if self.cycles_to_retire == 0 {
            match self.program[self.pc] {
                Op::Noop => (),
                Op::Addx(offset) => self.x += offset,
            }
            self.pc += 1;
        }
        ret
    }
}

fn part1(program: &Vec<Op>) -> i64 {
    let mut cpu = Cpu::new(program);
    let mut cycle = 0;
    let mut signal_strength_sum = 0;
    while let Some(x) = cpu.step() {
        cycle += 1;
        if cycle % 40 == 20 {
            signal_strength_sum += x * cycle;
        }
    }
    signal_strength_sum
}

fn part2_image(program: &Vec<Op>) -> String {
    let mut cpu = Cpu::new(program);
    let mut cycle = 0;
    let mut image = String::with_capacity(240);
    while let Some(x) = cpu.step() {
        let ch = if (x - cycle).abs() <= 1 { '#' } else { '.' };
        cycle += 1;
        image.push(ch);
        if cycle == 40 {
            cycle = 0;
            image.push('\n');
        }
    }
    image
}

fn part2(program: &Vec<Op>) -> String {
    ascii_bitmap::decode(&part2_image(program)).unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let program = parse(&puzzle_input);
    println!("{}", part1(&program));
    println!("{}", part2(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 13140);
    }

    #[test]
    fn test_part2() {
        const IMAGE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(&part2_image(&parse(EX)), IMAGE);
    }
}
