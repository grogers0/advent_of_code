use std::io::{self, Read};

fn part1(input: &str) -> String {
    let mut code = String::new();
    let mut pos = (0, 0);
    let keypad = [['1', '2', '3'],
                  ['4', '5', '6'],
                  ['7', '8', '9']];

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                'U' => if pos.1 > 0 { pos.1 -= 1 },
                'D' => if pos.1 < 2 { pos.1 += 1 },
                'L' => if pos.0 > 0 { pos.0 -= 1 },
                'R' => if pos.0 < 2 { pos.0 += 1 },
                _ => unreachable!()
            }
        }
        code.push(keypad[pos.1][pos.0]);
    }

    code
}

fn part2(input: &str) -> String {
    let mut code = String::new();
    let mut pos = (0, 2);
    let keypad = [[' ', ' ', '1', ' ', ' '],
                  [' ', '2', '3', '4', ' '],
                  ['5', '6', '7', '8', '9'],
                  [' ', 'A', 'B', 'C', ' '],
                  [' ', ' ', 'D', ' ', ' ']];

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                'U' => if pos.1 > 0 && keypad[pos.1 - 1][pos.0] != ' ' { pos.1 -= 1 },
                'D' => if pos.1 < 4 && keypad[pos.1 + 1][pos.0] != ' ' { pos.1 += 1 },
                'L' => if pos.0 > 0 && keypad[pos.1][pos.0 - 1] != ' ' { pos.0 -= 1 },
                'R' => if pos.0 < 4 && keypad[pos.1][pos.0 + 1] != ' ' { pos.0 += 1 },
                _ => unreachable!()
            }
        }
        code.push(keypad[pos.1][pos.0]);
    }

    code
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

    const EX: &str = "\
ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), "1985".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), "5DB3".to_string());
    }
}
