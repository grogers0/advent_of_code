use std::io::{self, Read};
use std::collections::HashMap;

fn parse(puzzle_input: &str) -> [Vec<u64>; 2] {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in puzzle_input.lines() {
        let mut sp = line.split_whitespace();
        left_list.push(sp.next().unwrap().parse::<u64>().unwrap());
        right_list.push(sp.next().unwrap().parse::<u64>().unwrap());
        assert!(sp.next().is_none());
    }
    [left_list, right_list]
}

fn abs_diff(a: u64, b: u64) -> u64 {
    if a > b { a - b } else { b - a }
}

fn part1([left_list, right_list]: &[Vec<u64>; 2]) -> u64 {
    let mut left_list = left_list.clone();
    left_list.sort_unstable();
    let mut right_list = right_list.clone();
    right_list.sort_unstable();

    let mut sum = 0;
    for i in 0..left_list.len() {
        sum += abs_diff(left_list[i], right_list[i]);
    }
    sum
}

fn part2([left_list, right_list]: &[Vec<u64>; 2]) -> usize {
    let mut right_list_counts = HashMap::new();
    for &right in right_list.iter() {
        right_list_counts.entry(right).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut sum = 0;
    for &left in left_list.iter() {
        sum += left as usize * *right_list_counts.get(&left).unwrap_or(&0);
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let lists = parse(&puzzle_input);
    println!("{}", part1(&lists));
    println!("{}", part2(&lists));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 31);
    }
}
