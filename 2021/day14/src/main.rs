use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Pair = [char; 2];

// The key insight is that we don't care about the ordering, only how many of each pair
struct Polymer {
    pair_counts: HashMap<Pair, u64>,
    last_char: char
}

impl Polymer {
    fn from_str(s: &str) -> Self {
        let mut pair_counts = HashMap::new();
        let mut char_iter = s.chars();
        let mut last_char = char_iter.next().unwrap();
        for ch in char_iter {
            pair_counts.entry([last_char, ch]).and_modify(|count| *count += 1).or_insert(1);
            last_char = ch;
        }
        Polymer { pair_counts, last_char }
    }

    fn step(self, insertion_rules: &HashMap<Pair, Vec<Pair>>) -> Polymer {
        let mut ret = Polymer { pair_counts: HashMap::new(), last_char: self.last_char };
        for (input_pair, input_count) in self.pair_counts {
            for output_pair in insertion_rules[&input_pair].iter() {
                ret.pair_counts.entry(*output_pair)
                    .and_modify(|output_count| *output_count += input_count)
                    .or_insert(input_count);
            }
        }
        ret
    }

    fn most_minus_least_common_elem(&self) -> u64 {
        let mut char_counts = HashMap::new();
        char_counts.insert(self.last_char, 1);
        for (&[a, _], &pair_count) in self.pair_counts.iter() {
            char_counts.entry(a)
                .and_modify(|char_count| *char_count += pair_count)
                .or_insert(pair_count);
        }

        let most_common = char_counts.values().max().unwrap();
        let least_common = char_counts.values().min().unwrap();
        most_common - least_common
    }

}

fn parse(puzzle_input: &str) -> (Polymer, HashMap<Pair, Vec<Pair>>) {
    let mut split_iter = puzzle_input.split("\n\n");
    let polymer_template = Polymer::from_str(split_iter.next().unwrap());
    let insertion_rules: HashMap<Pair, Vec<Pair>> = split_iter.next().unwrap().lines()
        .map(|line| {
            let mut split_iter = line.split(" -> ");
            let pair_str = split_iter.next().unwrap();
            let insert_str = split_iter.next().unwrap();

            let mut pair_chars = pair_str.chars();
            let mut insert_chars = insert_str.chars();
            let a = pair_chars.next().unwrap();
            let b = pair_chars.next().unwrap();
            assert!(pair_chars.next().is_none());
            let c = insert_chars.next().unwrap();
            assert!(insert_chars.next().is_none());
            ([a, b], vec![[a, c], [c, b]])
        })
        .collect();
    assert!(split_iter.next().is_none());

    // If not all pairs have a mapping, we'd need to insert noops, but they do
    let all_elements: HashSet<char> = insertion_rules.iter()
        .flat_map(|(inputs, outputs)| inputs.iter().chain(outputs.iter().flat_map(|pairs| pairs)))
        .cloned()
        .collect();
    assert_eq!(all_elements.len() * all_elements.len(), insertion_rules.len());
    (polymer_template, insertion_rules)
}

fn part1(puzzle_input: &str) -> u64 {
    let (mut polymer, insertion_rules) = parse(puzzle_input);
    for _ in 0..10 {
        polymer = polymer.step(&insertion_rules);
    }
    polymer.most_minus_least_common_elem()
}

fn part2(puzzle_input: &str) -> u64 {
    let (mut polymer, insertion_rules) = parse(puzzle_input);
    for _ in 0..40 {
        polymer = polymer.step(&insertion_rules);
    }
    polymer.most_minus_least_common_elem()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() {
        assert_eq!(1588, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2188189693529, part2(EX));
    }
}
