use std::collections::VecDeque;
use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

fn md5(input: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(input);
    digest.result_str()
}

fn stretched_md5(input: &str, additional: usize) -> String {
    let mut hash = md5(input);
    for _ in 0..additional {
        hash = md5(&hash);
    }
    hash
}

fn get_triplet(key: &str) -> Option<char> {
    let mut last_ch = '_';
    let mut repeats = 0;
    for ch in key.chars() {
        if ch == last_ch {
            repeats += 1;
            if repeats >= 3 { return Some(ch); }
        } else {
            last_ch = ch;
            repeats = 1;
        }
    }
    None
}

fn has_quintuplet(key: &str, ch: char) -> bool {
    let mut repeats = 0;
    for ch2 in key.chars() {
        if ch == ch2 {
            repeats += 1;
            if repeats >= 5 { return true; }
        } else {
            repeats = 0;
        }
    }
    false
}

fn calc(input: &str, stretch_factor: usize) -> usize {
    let salt = input.trim_end();
    let mut queue = VecDeque::new();
    let mut key_cnt = 0;
    for i in 0.. {
        let hash = stretched_md5(&format!("{}{}", salt, i), stretch_factor);
        queue.push_back(hash);
        if queue.len() <= 1000 {
            continue;
        }
        let hash = queue.pop_front().unwrap();
        if let Some(ch) = get_triplet(&hash) {
            if queue.iter().any(|h| has_quintuplet(h, ch)) {
                key_cnt += 1;
                if key_cnt >= 64 {
                    return i - 1000;
                }
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    calc(input, 0)
}

fn part2(input: &str) -> usize {
    calc(input, 2016)
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
        assert_eq!(part1("abc"), 22728);
    }

    // NOTE - Make sure to use cargo test --release
    #[test]
    fn test_part2() {
        assert_eq!(part2("abc"), 22551);
    }
}
