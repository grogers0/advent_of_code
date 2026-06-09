use std::io::{self, Read};
use std::ops::RangeInclusive;

fn parse(puzzle_input: &str) -> Vec<RangeInclusive<u64>> {
    let mut ret = vec![];
    for line in puzzle_input.trim_end().split(",") {
        let mut str_ids = line.split("-");
        let lower_id = str_ids.next().unwrap().parse::<u64>().unwrap();
        let upper_id = str_ids.next().unwrap().parse::<u64>().unwrap();
        assert!(str_ids.next().is_none());
        ret.push(lower_id..=upper_id);
    }
    ret
}

fn is_invalid_by_reps(id: &str, rep_len: usize) -> bool {
    if (id.len() % rep_len) != 0 { return false };
    let base = &id[0..rep_len];
    for i in 1..(id.len() / rep_len) {
        if base != &id[(rep_len*i)..(rep_len*(i+1))] {
            return false;
        }
    }
    true
}

fn part1(ranges: &[RangeInclusive<u64>]) -> u64 {
    let mut sum = 0;
    for range in ranges {
        for id_num in range.clone() {
            let id = id_num.to_string();
            if id.len() % 2 == 0 && is_invalid_by_reps(&id, id.len() / 2) {
                sum += id_num;
            }
        }
    }
    sum
}

fn part2(ranges: &[RangeInclusive<u64>]) -> u64 {
    fn is_invalid(id: &str) -> bool {
        for rep_len in 1..=(id.len() / 2) {
            if is_invalid_by_reps(id, rep_len) { return true };
        }
        false
    }
    let mut sum = 0;
    for range in ranges {
        for id_num in range.clone() {
            let id = id_num.to_string();
            if is_invalid(&id) {
                sum += id_num;
            }
        }
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let ranges = parse(&puzzle_input);
    println!("{}", part1(&ranges));
    println!("{}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 4174379265);
    }
}
