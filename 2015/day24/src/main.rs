use std::collections::BTreeSet;
use std::io::{self, Read};

fn parse(input: &str) -> BTreeSet<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn has_other_combinations(nums: &BTreeSet<u64>, target: u64, remaining_combos: u64) -> bool {
    fn calc(combo: BTreeSet<u64>, chose_nums: BTreeSet<u64>, all_nums: &BTreeSet<u64>, target: u64, remaining_combos: u64) -> bool {
        let combo_sum: u64 = combo.iter().sum();
        for num in chose_nums.iter() {
            let mut combo2 = combo.clone();
            combo2.insert(*num);

            if num + combo_sum == target {
                if remaining_combos <= 2 { return true }
                let nums_other = all_nums.difference(&combo2).cloned().collect();
                if has_other_combinations(&nums_other, target, remaining_combos - 1) { return true }
                return false;
            } else if num + combo_sum > target {
                return false;
            }

            let chose_nums2 = chose_nums.iter().filter(|n| *n > num).cloned().collect();
            if calc(combo2, chose_nums2, all_nums, target, remaining_combos) { return true }
        }
        false
    }
    calc(BTreeSet::new(), nums.clone(), nums, target, remaining_combos)
}

fn find_all_first_combinations(nums: &BTreeSet<u64>, target: u64, remaining_combos: u64) -> Vec<BTreeSet<u64>> {
    fn calc(combo: BTreeSet<u64>, chose_nums: BTreeSet<u64>, all_nums: &BTreeSet<u64>, target: u64, remaining_combos: u64, acc: &mut Vec<BTreeSet<u64>>) {
        let combo_sum: u64 = combo.iter().sum();
        for num in chose_nums.iter() {
            let mut combo2 = combo.clone();
            combo2.insert(*num);

            if num + combo_sum == target {
                let nums_other = all_nums.difference(&combo2).cloned().collect();
                if has_other_combinations(&nums_other, target, remaining_combos - 1) { acc.push(combo2); }
                continue;
            } else if num + combo_sum > target {
                continue;
            }

            let chose_nums2 = chose_nums.iter().filter(|n| *n > num).cloned().collect();
            calc(combo2, chose_nums2, all_nums, target, remaining_combos, acc);
        }
    }
    let mut ret = Vec::new();
    calc(BTreeSet::new(), nums.clone(), nums, target, remaining_combos, &mut ret);
    ret
}

fn quantum_entanglement(combo: &BTreeSet<u64>) -> u64 {
    combo.iter().product()
}

fn best_configuration(input: &str, num_groups: u64) -> u64 {
    let nums = parse(input);
    let target = nums.iter().sum::<u64>() / num_groups;
    let combos = find_all_first_combinations(&nums, target, num_groups);
    let min_len = combos.iter().map(|combo| combo.len()).min().unwrap();
    let combos: Vec<_> = combos.into_iter().filter(|combo| combo.len() == min_len).collect();
    let mut qes: Vec<_> = combos.iter().map(|combo| quantum_entanglement(combo)).collect();
    qes.sort();
    qes.into_iter().next().unwrap()
}

fn part1(input: &str) -> u64 {
    best_configuration(input, 3)
}

fn part2(input: &str) -> u64 {
    best_configuration(input, 4)
}

// Takes a minute to run, probably missing some obvious way to cut down the combinations visited,
// but this is fine.
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
1
2
3
4
5
7
8
9
10
11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 99);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 44);
    }
}
