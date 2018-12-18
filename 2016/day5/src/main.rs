use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

fn md5(input: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(input);
    digest.result_str()
}

const PASSWORD_LEN: usize = 8;

fn part1(input: &str) -> String {
    let mut cnt = 0;
    let mut password = String::new();
    for i in 0.. {
        let hex = md5(&format!("{}{}", input, i));
        if hex.chars().take(5).all(|ch| ch == '0') {
            password.push(hex.chars().nth(5).unwrap());
            cnt += 1;
            if cnt >= PASSWORD_LEN {
                break;
            }
        }
    }
    password
}

fn part2(input: &str) -> String {
    let mut password = vec!['_'; PASSWORD_LEN];
    for i in 0.. {
        let hex = md5(&format!("{}{}", input, i));
        if hex.chars().take(5).all(|ch| ch == '0') {
            match hex.chars().nth(5).unwrap().to_digit(10).map(|x| x as usize) {
                Some(pos) if pos < PASSWORD_LEN && password[pos] == '_' => {
                    password[pos] = hex.chars().nth(6).unwrap();
                    if password.iter().all(|ch| *ch != '_') {
                        break;
                    } else {
                        // Animation
                        eprintln!("{}", password.iter().collect::<String>());
                    }
                },
                _ => ()
            }
        }
    }
    password.iter().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_end(); // Ignore the newline

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE - make sure to use cargo test --release or this will take forever
    #[test]
    fn test_part1() {
        assert_eq!(part1("abc"), "18f47a30".to_string())
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abc"), "05ace8e3".to_string())
    }
}
