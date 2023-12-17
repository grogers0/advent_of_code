use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
enum Condition {
    Damaged,
    Operational,
    Unknown,
}

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

fn count_contiguous_no_unknowns(conditions: &[Condition]) -> Vec<usize> {
    let mut contig_vec = Vec::new();
    let mut curr_contig = 0;
    for cond in conditions {
        match cond {
            Condition::Damaged => {
                curr_contig += 1;
            },
            Condition::Operational => {
                if curr_contig > 0 {
                    contig_vec.push(curr_contig);
                    curr_contig = 0;
                }
            },
            Condition::Unknown => panic!(),
        }
    }
    if curr_contig > 0 {
        contig_vec.push(curr_contig);
    }
    contig_vec
}

fn count_arrangements(record: &ConditionRecord) -> u64 {
    fn recur(conditions: &mut [Condition], expected_contig: &[usize], i: usize) -> u64 {
        if i == conditions.len() {
            if count_contiguous_no_unknowns(conditions) == expected_contig {
                return 1;
            } else {
                return 0;
            }
        }
        if let Condition::Unknown = conditions[i] {
            let mut sum = 0;
            conditions[i] = Condition::Damaged;
            sum += recur(conditions, expected_contig, i+1);
            conditions[i] = Condition::Operational;
            sum += recur(conditions, expected_contig, i+1);
            conditions[i] = Condition::Unknown;
            sum
        } else {
            recur(conditions, expected_contig, i+1)
        }
    }
    let mut conditions = record.conditions.clone();
    recur(&mut conditions, &record.contiguous_damaged, 0)
}

fn part1(records: &[ConditionRecord]) -> u64 {
    records.iter().map(|record| count_arrangements(record)).sum()
}

fn part2(puzzle_input: &str) -> &str {
    "FIXME"
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let records = parse(&puzzle_input);
    println!("{}", part1(&records));
    println!("{}", part2(&puzzle_input));
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
        // FIXME
    }
}
