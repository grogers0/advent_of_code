use std::io::{self, Read};

fn snafu_to_decimal(s: &str) -> i64 {
    let mut ret = 0;
    for ch in s.chars() {
        let n = match ch {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        };
        ret = ret * 5 + n;
    }
    ret
}

fn decimal_to_snafu(mut n: i64) -> String {
    let mut reversed = Vec::new();
    while n != 0 {
        let ch = match n % 5 {
            0 => '0',
            1 => {
                n -= 1;
                '1'
            },
            2 => {
                n -= 2;
                '2'
            },
            3 => {
                n += 2;
                '='
            },
            4 => {
                n += 1;
                '-'
            },
            _ => unreachable!(),
        };
        reversed.push(ch);
        n /= 5;
    }
    let mut ret = String::with_capacity(reversed.len());
    for ch in reversed.iter().rev() {
        ret.push(*ch);
    }
    ret
}

fn part1(puzzle_input: &str) -> String {
    let sum = puzzle_input.trim_end().lines().map(|line| snafu_to_decimal(line)).sum();
    decimal_to_snafu(sum)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

const EX: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    fn test_round_trip(s: &str, n: i64) {
        assert_eq!(snafu_to_decimal(s), n);
        assert_eq!(&decimal_to_snafu(n), s);
    }

    #[test]
    fn test_part1() {
        test_round_trip("1=-0-2", 1747);
        test_round_trip("12111", 906);
        test_round_trip("2=0=", 198);
        test_round_trip("21", 11);
        test_round_trip("2=01", 201);
        test_round_trip("111", 31);
        test_round_trip("20012", 1257);
        test_round_trip("112", 32);
        test_round_trip("1=-1=", 353);
        test_round_trip("1-12", 107);
        test_round_trip("12", 7);
        test_round_trip("1=", 3);
        test_round_trip("122", 37);

        assert_eq!(part1(EX), "2=-1=0".to_string());
    }
}
