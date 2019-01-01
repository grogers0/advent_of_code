use std::io::{self, Read};

fn look_and_say(input: &str) -> String {
    let mut ret = String::new();
    let mut cnt = 0;
    let mut last_ch = '_';
    for ch in input.chars() {
        if ch == last_ch {
            cnt += 1;
        } else {
            if cnt != 0 {
                ret.push_str(&format!("{}", cnt));
                ret.push(last_ch);
            }
            last_ch = ch;
            cnt = 1;
        }
    }
    ret.push_str(&format!("{}", cnt));
    ret.push(last_ch);
    ret
}

fn iterate_look_and_say(input: &str, rounds: usize) -> usize {
    let mut seq = input.trim_end().to_string();
    for _ in 0..rounds {
        seq = look_and_say(&seq);
    }
    seq.len()
}

fn part1(input: &str) -> usize {
    iterate_look_and_say(input, 40)
}

fn part2(input: &str) -> usize {
    iterate_look_and_say(input, 50)
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
        assert_eq!(&look_and_say("1"), "11");
        assert_eq!(&look_and_say("11"), "21");
        assert_eq!(&look_and_say("21"), "1211");
        assert_eq!(&look_and_say("1211"), "111221");
        assert_eq!(&look_and_say("111221"), "312211");
    }
}
