use std::io::{self, Read};

use day12_2016::*;

fn part1(input: &str) -> i64 {
    execute_with_initial_state(input, |registers| { registers.insert("a".to_string(), 7); })
}

// TODO - This only took a few minutes to run so I didn't bother actually hand optimizing the
// assembly code as in other similar challenges.
fn part2(input: &str) -> i64 {
    execute_with_initial_state(input, |registers| { registers.insert("a".to_string(), 12); })
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
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 3);
    }
}
