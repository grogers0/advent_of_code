use std::collections::HashSet;
use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<u64> {
    puzzle_input.lines().map(|line| line.parse().unwrap()).collect()
}

fn pair_sums_to(preamble_nums: &HashSet<u64>, target: u64) -> bool {
    for &v1 in preamble_nums {
        if target != v1*2 && v1 < target && preamble_nums.contains(&(target - v1)) {
            return true
        }
    }
    false
}

fn part1(nums: &Vec<u64>, preamble_len: usize) -> u64 {
    let mut preamble_nums: HashSet<u64> =
        nums[0..preamble_len].iter().cloned().collect();
    for i in preamble_len .. nums.len() {
        if !pair_sums_to(&preamble_nums, nums[i]) {
            return nums[i]
        }
        preamble_nums.remove(&nums[i-preamble_len]);
        preamble_nums.insert(nums[i]);
    }
    panic!()
}

fn part2(nums: &Vec<u64>, target: u64) -> u64 {
    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in 1..(nums.len()-i) {
            sum += nums[i+j];
            if sum == target {
                return nums[i..(i+j+1)].iter().min().unwrap() +
                    nums[i..(i+j+1)].iter().max().unwrap();
            } else if sum > target {
                break
            }
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let nums = parse(&puzzle_input);

    let first_invalid = part1(&nums, 25);
    println!("{}", first_invalid);
    println!("{}", part2(&nums, first_invalid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part1() {
        assert_eq!(127, part1(&parse(EX), 5));
    }

    #[test]
    fn test_part2() {
        assert_eq!(62, part2(&parse(EX), 127));
    }
}
