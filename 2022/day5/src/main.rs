use std::io::{self, Read};
use regex::Regex;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(puzzle_input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut input_sections = puzzle_input.trim_end().split("\n\n");
    let mut stacks = Vec::new();
    let mut lines = input_sections.next().unwrap().lines().rev();
    stacks.resize((lines.next().unwrap().chars().count() + 1) / 4, Vec::new());
    for line in lines {
        assert_eq!(4 * stacks.len(), line.chars().count() + 1);
        for (i, ch) in line.chars().enumerate() {
            if i % 4 != 1 { continue }
            if ch == ' ' { continue }
            stacks[i / 4].push(ch);
        }
    }

    let move_re = Regex::new("^move (\\d+) from (\\d+) to (\\d+)$").unwrap();
    let moves = input_sections.next().unwrap().lines().map(|line| {
        let cap = move_re.captures(line).unwrap();
        Move {
            count: cap[1].parse().unwrap(),
            from: cap[2].parse::<usize>().unwrap() - 1,
            to: cap[3].parse::<usize>().unwrap() - 1,
        }
    }).collect();

    assert!(input_sections.next().is_none());

    (stacks, moves)
}

fn stack_tops(stacks: &[Vec<char>]) -> String {
    let mut ret = String::new();
    for stack in stacks {
        ret.push(stack[stack.len() - 1]);
    }
    ret
}

fn part1(stacks: &mut [Vec<char>], moves: &[Move]) -> String {
    for m in moves {
        for _ in 0..m.count {
            let ch = stacks[m.from].pop().unwrap();
            stacks[m.to].push(ch);
        }
    }
    stack_tops(&stacks)
}

fn part2(stacks: &mut [Vec<char>], moves: &[Move]) -> String {
    for m in moves {
        for i in 0..m.count {
            let from_idx = stacks[m.from].len() - m.count + i;
            stacks[m.to].push(stacks[m.from][from_idx]);
        }
        for _ in 0..m.count {
            stacks[m.from].pop().unwrap();
        }
    }
    stack_tops(&stacks)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (mut stacks, moves) = parse(&puzzle_input);
    println!("{}", part1(&mut stacks.clone(), &moves));
    println!("{}", part2(&mut stacks, &moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        let (mut stacks, moves) = parse(EX);
        assert_eq!(&part1(&mut stacks, &moves), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (mut stacks, moves) = parse(EX);
        assert_eq!(&part2(&mut stacks, &moves), "MCD");
    }
}
