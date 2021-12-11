use std::io::{self, Read};

// An incomplete line will return the autocomplete characters as Ok(str), a
// corrupted line will return the first incorrect char as Err(char)
fn detect_corruption(line: &str) -> Result<String, char> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if stack.is_empty() || *stack.last().unwrap() != ch {
                    return Err(ch);
                } else {
                    stack.pop();
                }
            },
            _ => panic!()
        }
    }
    let mut ret = String::new();
    while let Some(ch) = stack.pop() {
        ret.push(ch);
    }
    Ok(ret)
}

fn part1(puzzle_input: &str) -> u32 {
    puzzle_input.lines()
        .map(detect_corruption)
        .filter_map(|res| match res { Ok(_) => None, Err(ch) => Some(ch) })
        .map(|ch| match ch {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!()
        })
        .sum()
}

fn part2(puzzle_input: &str) -> u64 {
    let mut scores: Vec<u64> = puzzle_input.lines()
        .map(detect_corruption)
        .filter_map(|res| match res { Ok(s) => Some(s), Err(_) => None })
        .map(|s| {
            let mut score = 0;
            for ch in s.chars() {
                score = score * 5 + match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!()
                };
            }
            score
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(26397, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, part2(EX));
    }
}
