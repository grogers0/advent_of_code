use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<u16> {
    let mut ret: Vec<u16> = puzzle_input.lines().map(|line| line.parse().unwrap()).collect();
    ret.sort();
    ret
}

fn part1(sorted_adapters: &Vec<u16>) -> u64 {
    let mut cnt_one = 0;
    let mut cnt_three = 1; // Device adapter always three higher than then max pf the adapters
    let mut last_joltage = 0;
    for &adapter_joltage in sorted_adapters {
        let diff = adapter_joltage - last_joltage;
        last_joltage = adapter_joltage;
        match diff {
            1 => cnt_one += 1,
            3 => cnt_three += 1,
            _ => panic!()
        }
    }
    cnt_one * cnt_three
}

fn part2(sorted_adapters: &Vec<u16>) -> u64 {
    let max_adapter = sorted_adapters[sorted_adapters.len() - 1] as usize;
    let mut combos = vec![0; max_adapter+1];
    combos[0] = 1;
    for &joltage in sorted_adapters {
        let joltage = joltage as usize;
        let mut sum = 0;
        for i in 1..=3 {
            if i > joltage { break }
            sum += combos[joltage - i];
        }
        combos[joltage] = sum;
    }
    combos[max_adapter]
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let sorted_adapters = parse(&puzzle_input);

    println!("{}", part1(&sorted_adapters));
    println!("{}", part2(&sorted_adapters));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "16
10
15
5
1
11
7
19
6
12
4";
    const EX2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() {
        assert_eq!(7*5, part1(&parse(EX1)));
        assert_eq!(22*10, part1(&parse(EX2)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2(&parse(EX1)));
        assert_eq!(19208, part2(&parse(EX2)));
    }
}
