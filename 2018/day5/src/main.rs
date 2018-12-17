use std::io::{self, Read};

fn should_react(a: char, b: char) -> bool {
    assert!(a.is_ascii() && b.is_ascii());
    if !a.is_alphabetic() || !b.is_alphabetic() {
        false
    } else if a.is_lowercase() == b.is_lowercase()  {
        false
    } else {
        a.to_ascii_lowercase() == b.to_ascii_lowercase()
    }
}

fn fully_react(mut polymer: String) -> String {
    let mut stop = false;
    while !stop {
        stop = true;
        let mut output = String::new();

        let mut prev_ch = None;
        for ch in polymer.chars() {
            if let Some(ch2) = prev_ch {
                if should_react(ch, ch2) {
                    prev_ch = None;
                    stop = false;
                } else {
                    output.push(ch2);
                    prev_ch = Some(ch);
                }
            } else {
                prev_ch = Some(ch);
            }
        }
        prev_ch.map(|ch| output.push(ch));

        polymer = output;
    }

    polymer
}

fn part1(input: &str) -> usize {
    fully_react(input.to_string()).chars().count()
}

fn part2(input: &str) -> usize {
    ('a' as u8 ..= 'z' as u8).map(|remove_ch_u8| {
        let remove_ch = remove_ch_u8 as char;
        let mut polymer = input.to_string();
        polymer.retain(|ch| ch != remove_ch && ch != remove_ch.to_ascii_uppercase());
        fully_react(polymer).chars().count()
    }).min().unwrap()
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

    const EX: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 4);
    }

}
