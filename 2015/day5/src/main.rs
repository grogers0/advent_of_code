use std::io::{self, Read};

fn is_vowel(ch: char) -> bool {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn has_repeated_char(s: &str) -> bool {
    if s.len() == 0 { return false }
    let mut chars_iter = s.chars();
    let mut last_ch = chars_iter.next().unwrap();
    for ch in chars_iter {
        if ch == last_ch { return true }
        last_ch = ch;
    }
    false
}

fn has_repeated_pair(s: &str) -> bool {
    for i in 0..s.len()-1 {
        let pair = &s[i..i+2];
        if s[i+2..].contains(pair) { return true }
    }
    false
}

fn has_aba(s: &str) -> bool {
    if s.len() <= 1 { return false }
    let mut chars_iter = s.chars();
    let mut a = chars_iter.next().unwrap();
    let mut b = chars_iter.next().unwrap();
    for ch in chars_iter {
        if ch == a { return true }
        a = b;
        b = ch;
    }
    false
}

fn part1(input: &str) -> usize {
    fn is_nice(s: &str) -> bool {
        s.chars().filter(|ch| is_vowel(*ch)).count() >= 3
            && has_repeated_char(s)
            && !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
    }
    input.lines()
        .filter(|line| is_nice(*line))
        .count()
}

fn part2(input: &str) -> usize {
    fn is_nice(s: &str) -> bool {
        has_repeated_pair(s) && has_aba(s)
    }
    input.lines()
        .filter(|line| is_nice(*line))
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
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
        assert_eq!(part1("aaa"), 1);
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("xyxy"), 1);
        assert_eq!(part2("aaa"), 0);
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }
}
