use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Condition {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Debug)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    contiguous_damaged: Vec<usize>,

}

fn parse(puzzle_input: &str) -> Vec<ConditionRecord> {
    puzzle_input.lines().map(|line| {
        let mut sp_iter = line.split(" ");
        let mut conditions = Vec::new();
        for ch in sp_iter.next().unwrap().chars() {
            let cond = match ch {
                '#' => Condition::Damaged,
                '.' => Condition::Operational,
                '?' => Condition::Unknown,
                _ => panic!(),
            };
            conditions.push(cond);
        }
        let contiguous_damaged = sp_iter.next().unwrap().split(",")
            .map(|token| token.parse().unwrap()).collect();
        ConditionRecord { conditions, contiguous_damaged }
    }).collect()
}

fn count_arrangements(record: &ConditionRecord) -> u64 {
    fn recur(conditions: &mut [Condition], mut expected_contiguous: &[usize], prior_contiguous: usize,
        memo: &mut HashMap<(Vec<Condition>, Vec<usize>, usize), u64>) -> u64 {
        if conditions.is_empty() {
            if prior_contiguous > 0 {
                if expected_contiguous.is_empty() {
                    return 0;
                } else if prior_contiguous != expected_contiguous[0] {
                    return 0;
                }
                expected_contiguous = &expected_contiguous[1..];
            }
            return if expected_contiguous.is_empty() { 1 } else { 0 };
        } else if let Some(ret) = memo.get(&(conditions.to_vec(), expected_contiguous.to_vec(), prior_contiguous)) {
            return *ret;
        }

        let ret = match conditions[0] {
            Condition::Damaged => recur(&mut conditions[1..], expected_contiguous, prior_contiguous + 1, memo),
            Condition::Operational => {
                if prior_contiguous > 0 {
                    if expected_contiguous.is_empty() {
                        return 0;
                    } else if prior_contiguous != expected_contiguous[0] {
                        return 0;
                    }
                    expected_contiguous = &expected_contiguous[1..];
                }
                recur(&mut conditions[1..], expected_contiguous, 0, memo)
            },
            Condition::Unknown => {
                let mut sum = 0;
                conditions[0] = Condition::Damaged;
                sum += recur(conditions, expected_contiguous, prior_contiguous, memo);
                conditions[0] = Condition::Operational;
                sum += recur(conditions, expected_contiguous, prior_contiguous, memo);
                conditions[0] = Condition::Unknown;
                sum
            },
        };
        memo.insert((conditions.to_vec(), expected_contiguous.to_vec(), prior_contiguous), ret);
        ret
    }
    let mut conditions = record.conditions.clone();
    let mut memo = HashMap::new();
    recur(&mut conditions, &record.contiguous_damaged, 0, &mut memo)
}

fn part1(records: &[ConditionRecord]) -> u64 {
    records.iter().map(|record| count_arrangements(record)).sum()
}

fn part2(records: &[ConditionRecord]) -> u64 {
    let records: Vec<_> = records.iter().map(|record| {
        let mut conditions = Vec::with_capacity(5 * record.conditions.len() + 4);
        let mut contiguous_damaged = Vec::with_capacity(5 * record.contiguous_damaged.len());
        for i in 0..5 {
            if i > 0 { conditions.push(Condition::Unknown); }
            for &c in &record.conditions {
                conditions.push(c);
            }
            for &d in &record.contiguous_damaged {
                contiguous_damaged.push(d);
            }
        }
        ConditionRecord { conditions, contiguous_damaged }
    }).collect();
    records.iter().map(|record| count_arrangements(record)).sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let records = parse(&puzzle_input);
    println!("{}", part1(&records));
    println!("{}", part2(&records));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 525152);
    }
}
