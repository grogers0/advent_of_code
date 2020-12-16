use std::collections::HashMap;
use std::io::{self, Read};

// TODO - Is there a smarter way to do this for the 30 million case? It does take a few seconds.
fn iterate(starting_nums: &Vec<usize>, iterations: usize) -> usize {
    let mut last = 0;
    let mut times = HashMap::new();
    let mut counts = HashMap::new();
    for turn in 0..iterations {
        let next = if turn < starting_nums.len() {
            starting_nums[turn]
        } else if *counts.get(&last).unwrap() == 1 {
            0
        } else {
            turn - times.get(&last).unwrap()
        };
        if turn > 0 { times.insert(last, turn); }
        last = next;
        *counts.entry(last).or_insert(0) += 1;
    }
    last
}

fn part1(starting_nums: &Vec<usize>) -> usize {
    iterate(starting_nums, 2020)
}

fn part2(starting_nums: &Vec<usize>) -> usize {
    iterate(starting_nums, 30_000_000)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let starting_nums = puzzle_input.trim_end().split(",").map(|n| n.parse().unwrap()).collect();

    println!("{}", part1(&starting_nums));
    println!("{}", part2(&starting_nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(436, part1(&vec![0,3,6]));
        assert_eq!(1, part1(&vec![1,3,2]));
        assert_eq!(10, part1(&vec![2,1,3]));
        assert_eq!(27, part1(&vec![1,2,3]));
        assert_eq!(78, part1(&vec![2,3,1]));
        assert_eq!(438, part1(&vec![3,2,1]));
        assert_eq!(1836, part1(&vec![3,1,2]));
    }

    // NOTE - separate tests so they run in parallel to speed it up
    #[test]
    fn test_part2_1() {
        assert_eq!(175594, part2(&vec![0,3,6]));
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(2578, part2(&vec![1,3,2]));
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(3544142, part2(&vec![2,1,3]));
    }
    #[test]
    fn test_part2_4() {
        assert_eq!(261214, part2(&vec![1,2,3]));
    }
    #[test]
    fn test_part2_5() {
        assert_eq!(6895259, part2(&vec![2,3,1]));
    }
    #[test]
    fn test_part2_6() {
        assert_eq!(18, part2(&vec![3,2,1]));
    }
    #[test]
    fn test_part2_7() {
        assert_eq!(362, part2(&vec![3,1,2]));
    }
}
