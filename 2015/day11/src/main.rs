use std::io::{self, Read};

fn increment_str(s: &mut str) {
    fn next(ch: &mut u8) -> bool {
        if *ch as char == 'z' {
            *ch = 'a' as u8;
            true
        } else {
            *ch += 1;
            false
        }
    }

    let bytes = unsafe { s.as_bytes_mut() };
    for i in (0..bytes.len()).rev() {
        if !next(&mut bytes[i]) { break }
    }
}

fn has_increasing(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 0..s.len()-2 {
        if bytes[i]+1 == bytes[i+1] && bytes[i]+2 == bytes[i+2] {
            return true;
        }
    }
    false
}

fn has_confusing_chars(s: &str) -> bool {
    for ch in s.chars() {
        match ch {
            'i' | 'o' | 'l' => return true,
            _ => ()
        }
    }
    false
}

fn has_two_pairs(s: &str) -> bool {
    let mut cnt = 0;
    let mut last_ch = '_';
    for ch in s.chars() {
        if ch == last_ch {
            cnt += 1;
            last_ch = '_';
        } else {
            last_ch = ch;
        }
    }
    cnt >= 2
}

fn is_password_allowed(s: &str) -> bool {
    has_increasing(s) && !has_confusing_chars(s) && has_two_pairs(s)
}

fn next_allowed_password(password: &mut str) {
    while {
        increment_str(password);

        !is_password_allowed(password)
    } {}
}

fn part1(input: &str) -> String {
    let mut password = input.trim_end().to_string();
    next_allowed_password(&mut password);
    password
}

fn part2(input: &str) -> String {
    let mut password = input.trim_end().to_string();
    next_allowed_password(&mut password);
    next_allowed_password(&mut password);
    password
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
        assert!(!is_password_allowed("hijklmmn"));
        assert!(!is_password_allowed("abbceffg"));
        assert!(!is_password_allowed("abbcegjk"));
        assert_eq!(&part1("abcdefgh"), "abcdffaa");
        assert_eq!(&part1("ghijklmn"), "ghjaabcc");
    }
}
