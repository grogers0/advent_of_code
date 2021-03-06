use std::io::{self, Read};

use day12_2016::*;

fn part1(input: &str) -> i64 {
    execute_with_initial_state(input, |_| {})
}

fn part2(input: &str) -> i64 {
    execute_with_initial_state(input, |registers| { registers.insert("c".to_string(), 1); })
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
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 42);
    }
}
