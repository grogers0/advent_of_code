use std::collections::BTreeMap;
use std::io::{self, Read};

fn letter_counts(input: &str) -> BTreeMap<char,u32> {
    let mut counts = BTreeMap::new();
    for ch in input.chars() {
        counts.entry(ch)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }
    counts
}

fn part1(input: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let counts = letter_counts(line);
        if counts.values().any(|cnt| *cnt == 2) { twos += 1 }
        if counts.values().any(|cnt| *cnt == 3) { threes += 1 }
    }
    twos * threes
}

fn part2(input: &str) -> String {
    let mut seen = BTreeMap::new();
    for line in input.lines() {
        for i in 0..line.chars().count() {
            let s: String = line.chars()
                .take(i)
                .chain(line.chars().skip(i + 1))
                .collect();
            if let Some(orig_line) = seen.get(&s) {
                if orig_line != &line { return s }
            }
            seen.insert(s, line);
        }
    }
    panic!("failed to find a matching string with one difference");
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
        let ex = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

        assert_eq!(part1(ex), 12);
    }

    #[test]
    fn test_part2() {
        let ex = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

        assert_eq!(part2(ex), "fgij".to_string());
    }

    #[test]
    #[should_panic]
    fn test_part2_panic() {
        part2("");
    }
}
