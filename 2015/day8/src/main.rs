use std::io::{self, Read};

fn unquoted_len(input: &str) -> usize {
    assert!(&input[0..1] == "\"" && &input[input.len()-1..] == "\"");
    let mut len = 0;
    let mut chars_iter = input[1..input.len()-1].chars();
    while let Some(ch) = chars_iter.next() {
        if ch == '\\' {
            if chars_iter.next().unwrap() == 'x' {
                chars_iter.next(); chars_iter.next();
            }
        }
        len += 1;
    }
    len
}

fn quoted_len(input: &str) -> usize {
    let mut len = 2; // Start and end quotes
    for ch in input.chars() {
        if ch == '\\' {
            len += 2;
        } else if ch == '"' {
            len += 2;
        } else {
            len += 1;
        }
    }
    len
}

fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            line.len() - unquoted_len(line)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| {
            quoted_len(line) - line.len()
        })
        .sum()
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

    const EX: &str =
r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 19);
    }
}
