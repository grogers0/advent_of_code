use std::collections::BTreeMap;
use std::io::{self, Read};

fn parse_containers(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn sum_combo(containers: &[u32], combo: u64) -> u32 {
    let mut sum = 0;
    for i in 0..containers.len() {
        if combo & (1 << i) != 0 {
            sum += containers[i];
        }
    }
    sum
}

fn matching_combos_by_size(input: &str, total: u32) -> BTreeMap<u32, u64> {
    let containers = parse_containers(input);
    assert!(containers.len() < 64);
    let mut counts = BTreeMap::new();
    for combo in 0..(1 << containers.len()) {
        if sum_combo(&containers, combo) == total {
            counts.entry(combo.count_ones())
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);
        }
    }
    counts
}

fn num_matching_combos(input: &str, total: u32) -> u64 {
    matching_combos_by_size(input, total).values().sum()
}

fn num_combos_of_min_containers(input: &str, total: u32) -> u64 {
    *matching_combos_by_size(input, total).values().next().unwrap()
}

fn part1(input: &str) -> u64 {
    num_matching_combos(input, 150)
}

fn part2(input: &str) -> u64 {
    num_combos_of_min_containers(input, 150)
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
20
15
10
5
5";

    #[test]
    fn test_part1() {
        assert_eq!(num_matching_combos(EX, 25), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(num_combos_of_min_containers(EX, 25), 3);
    }
}
