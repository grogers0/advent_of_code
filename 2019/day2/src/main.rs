use std::io::{self, Read};

fn parse(input: &str) -> Vec<usize> {
    input.trim().split(",")
        .map(|chunk| chunk.parse().unwrap())
        .collect()
}

fn run(mut program: Vec<usize>) -> Vec<usize> {
    let mut pc = 0;
    loop {
        match program[pc] {
            1 => {
                let store_offset = program[pc+3];
                program[store_offset] = program[program[pc+1]] + program[program[pc+2]];
                pc += 4;
            },
            2 => {
                let store_offset = program[pc+3];
                program[store_offset] = program[program[pc+1]] * program[program[pc+2]];
                pc += 4;
            },
            99 => break,
            _ => panic!()
        }
    }
    program
}

fn part1(input: &str) -> usize {
    let mut program = parse(input);
    program[1] = 12;
    program[2] = 2;
    let program = run(program);
    program[0]
}

fn part2(input: &str) -> usize {
    let input_program = parse(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input_program.clone();
            program[1] = noun;
            program[2] = verb;
            if run(program)[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!();
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
        assert_eq!(run(parse("1,9,10,3,2,3,11,0,99,30,40,50"))[0], 3500);
        assert_eq!(run(parse("1,0,0,0,99"))[0], 2);
        assert_eq!(run(parse("2,3,0,3,99"))[3], 6);
        assert_eq!(run(parse("2,3,0,3,99"))[3], 6);
        assert_eq!(run(parse("2,4,4,5,99,0"))[5], 9801);
        assert_eq!(run(parse("1,1,1,4,99,5,6,0,99"))[0], 30);
    }
}
