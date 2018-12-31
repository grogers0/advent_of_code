use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

fn md5(input: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(input);
    digest.result_str()
}

fn first_md5_suffix_with_leading_zeros(input: &str, num_zeros: usize) -> usize {
    for i in 1.. {
        let md5 = md5(&format!("{}{}", input.trim_end(), i));
        if md5.chars().take(num_zeros).all(|ch| ch == '0') {
            return i;
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    first_md5_suffix_with_leading_zeros(input, 5)
}

fn part2(input: &str) -> usize {
    first_md5_suffix_with_leading_zeros(input, 6)
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

    // NOTE - make sure to use cargo test --release or it will take forever
    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
