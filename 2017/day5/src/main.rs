use std::io::{self, Read};

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let mut instructions = parse(input);
    let mut offset = 0i32;
    let mut steps = 0;
    while offset >= 0 && offset < instructions.len() as i32 {
        steps += 1;
        let new_offset = instructions[offset as usize] + offset;
        instructions[offset as usize] += 1;
        offset = new_offset;
    }
    steps
}

fn part2(input: &str) -> usize {
    let mut instructions = parse(input);
    let mut offset = 0i32;
    let mut steps = 0;
    while offset >= 0 && offset < instructions.len() as i32 {
        steps += 1;
        let new_offset = instructions[offset as usize] + offset;
        if instructions[offset as usize] >= 3 {
            instructions[offset as usize] -= 1;
        } else {
            instructions[offset as usize] += 1;
        }
        offset = new_offset;
    }
    steps
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
0
3
0
1
-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 10);
    }

}
