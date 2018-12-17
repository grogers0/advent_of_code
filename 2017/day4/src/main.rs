use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

fn valid_passphrase_part1(phrase: &str) -> bool {
    let words: Vec<String> = phrase.split(" ").map(|w| w.to_string()).collect();
    let deduped: BTreeSet<String> = words.iter().map(|w| w.clone()).collect();
    words.len() == deduped.len()
}

fn part1(input: &str) -> usize {
    input.lines()
        .filter(|line| valid_passphrase_part1(line))
        .count()
}

fn count_letters(word: &str) -> BTreeMap<char, usize> {
    let mut letters = BTreeMap::new();
    for ch in word.chars() {
        letters.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    letters
}

fn valid_passphrase_part2(phrase: &str) -> bool {
    let words: Vec<BTreeMap<char, usize>> = phrase.split(" ").map(|w| count_letters(w)).collect();
    let deduped: BTreeSet<BTreeMap<char, usize>> = words.iter().map(|w| w.clone()).collect();
    words.len() == deduped.len()
}


fn part2(input: &str) -> usize {
    input.lines()
        .filter(|line| valid_passphrase_part2(line))
        .count()
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

    #[test]
    fn test_part1() {
        assert!(valid_passphrase_part1("aa bb cc dd ee"));
        assert!(!valid_passphrase_part1("aa bb cc dd aa"));
        assert!(valid_passphrase_part1("aa bb cc dd aaa"));
    }

    #[test]
    fn test_part2() {
        assert!(valid_passphrase_part2("abcde fghij"));
        assert!(!valid_passphrase_part2("abcde xyz ecdab"));
        assert!(valid_passphrase_part2("a ab abc abd abf abj"));
        assert!(valid_passphrase_part2("iiii oiii ooii oooi oooo"));
        assert!(!valid_passphrase_part2("oiii ioii iioi iiio"));
    }

}
