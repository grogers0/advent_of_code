use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<i64> {
    puzzle_input.trim_end().lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(nums: &[i64], positions: &mut Vec<usize>) {
    let len = nums.len();
    debug_assert_eq!(len, positions.len());
    let removed_len = len as i64 - 1;

    for num_idx in 0..len {
        let num = nums[num_idx];
        let pos_idx = positions.iter().cloned().enumerate()
            .find(|(_, v)| *v == num_idx).unwrap().0;
        positions.remove(pos_idx);
        let pos_idx = ((((pos_idx as i64 + num) % removed_len) + removed_len) % removed_len) as usize;
        positions.insert(pos_idx, num_idx);
    }
}

fn coord(nums: &[i64], positions: &[usize]) -> i64 {
    let len = nums.len();
    debug_assert_eq!(len, positions.len());

    let zero_num_idx = nums.iter().cloned().enumerate()
        .find(|(_, num)| *num == 0).unwrap().0;
    let zero_pos_idx = positions.iter().cloned().enumerate()
        .find(|(_, idx2)| zero_num_idx == *idx2).unwrap().0;

    [1000, 2000, 3000].into_iter()
        .map(|offset| nums[positions[(zero_pos_idx + offset) % len]])
        .sum()
}

fn part1(nums: &[i64]) -> i64 {
    let mut positions: Vec<_> = (0..nums.len()).into_iter().collect();

    mix(&nums, &mut positions);
    coord(&nums, &positions)
}

fn part2(nums: &[i64]) -> i64 {
    let nums: Vec<_> = nums.iter().map(|num| num * 811589153).collect();
    let mut positions: Vec<_> = (0..nums.len()).into_iter().collect();

    for _ in 0..10 {
        mix(&nums, &mut positions);
    }
    coord(&nums, &positions)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let nums = parse(&puzzle_input);
    println!("{}", part1(&nums));
    println!("{}", part2(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 1623178306);
    }
}
