use std::io::{self, Read};

use regex::Regex;

enum Op {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize)
}

fn parse(input: &str) -> Vec<Op> {
    let swap_positions_re = Regex::new("^swap position (\\d+) with position (\\d+)$").unwrap();
    let swap_letters_re = Regex::new("^swap letter ([a-z]) with letter ([a-z])$").unwrap();
    let rotate_left_re = Regex::new("^rotate left (\\d+) steps?$").unwrap();
    let rotate_right_re = Regex::new("^rotate right (\\d+) steps?$").unwrap();
    let rotate_letter_re = Regex::new("^rotate based on position of letter ([a-z])$").unwrap();
    let reverse_re = Regex::new("^reverse positions (\\d+) through (\\d+)$").unwrap();
    let move_re = Regex::new("^move position (\\d+) to position (\\d+)$").unwrap();

    input.lines().map(|line| {
        if let Some(cap) = swap_positions_re.captures(line) {
            Op::SwapPositions(cap[1].parse().unwrap(), cap[2].parse().unwrap())
        } else if let Some(cap) = swap_letters_re.captures(line) {
            Op::SwapLetters(cap[1].chars().next().unwrap(), cap[2].chars().next().unwrap())
        } else if let Some(cap) = rotate_left_re.captures(line) {
            Op::RotateLeft(cap[1].parse().unwrap())
        } else if let Some(cap) = rotate_right_re.captures(line) {
            Op::RotateRight(cap[1].parse().unwrap())
        } else if let Some(cap) = rotate_letter_re.captures(line) {
            Op::RotateLetter(cap[1].chars().next().unwrap())
        } else if let Some(cap) = reverse_re.captures(line) {
            Op::Reverse(cap[1].parse().unwrap(), cap[2].parse().unwrap())
        } else if let Some(cap) = move_re.captures(line) {
            Op::Move(cap[1].parse().unwrap(), cap[2].parse().unwrap())
        } else {
            println!("{}", line);
            unimplemented!()
        }
    }).collect()
}

fn swap_positions(chars: &mut Vec<char>, x: usize, y: usize) {
    let tmp = chars[x];
    chars[x] = chars[y];
    chars[y] = tmp;
}

fn swap_letters(chars: &mut Vec<char>, x: char, y: char) {
    for ch in chars.iter_mut() {
        if *ch == x { *ch = y; }
        else if *ch == y { *ch = x; }
    }
}

fn rotate_left(chars: &mut Vec<char>, n: usize) {
    chars[..n].reverse();
    chars[n..].reverse();
    chars.reverse();
}

fn rotate_right(chars: &mut Vec<char>, n: usize) {
    chars.reverse();
    chars[..n].reverse();
    chars[n..].reverse();
}

fn rotate_letter(chars: &mut Vec<char>, x: char) {
    let pos = chars.iter().position(|ch| *ch == x).unwrap();
    let mut shift = pos + 1;
    if pos >= 4 { shift += 1; }
    rotate_right(chars, shift % chars.len());
}

fn unrotate_letter(chars: &mut Vec<char>, x: char) {
    let pos = chars.iter().position(|ch| *ch == x).unwrap();
    let mut saved_shift = None;
    for i in 0..chars.len() {
        let mut shift = i + 1;
        if i >= 4 { shift += 1; }
        if (i + shift) % chars.len() == pos {
            if let Some(shift2) = saved_shift {
                // NOTE - not all strings can be unrotated uniquely, since multiple inputs can map
                // to the same output. The actual puzzle input can be uniquely unscrambled, but the
                // part1 example cannot.
                panic!(format!("{} cannot be unrotated by letter {}, both shifts of {} and {} produce a valid result",
                               chars.iter().cloned().collect::<String>(), x, shift, shift2));
            }
            saved_shift = Some(shift % chars.len());
        }
    }
    rotate_left(chars, saved_shift.unwrap());
}

fn move_pos(chars: &mut Vec<char>, x: usize, y: usize) {
    let ch = chars.remove(x);
    chars.insert(y, ch);
}

fn scramble(startstr: &str, operations: &Vec<Op>) -> String {
    let mut chars: Vec<_> = startstr.chars().collect();
    for op in operations.iter() {
        match *op {
            Op::SwapPositions(x, y) => swap_positions(&mut chars, x, y),
            Op::SwapLetters(x, y) => swap_letters(&mut chars, x, y),
            Op::RotateLeft(x) => rotate_left(&mut chars, x),
            Op::RotateRight(x) => rotate_right(&mut chars, x),
            Op::RotateLetter(x) => rotate_letter(&mut chars, x),
            Op::Reverse(x, y) => chars[x..=y].reverse(),
            Op::Move(x, y) => move_pos(&mut chars, x, y)
        }
    }
    chars.into_iter().collect()
}

fn part1(operations: &Vec<Op>) -> String {
    scramble("abcdefgh", operations)
}

fn unscramble(endstr: &str, operations: &Vec<Op>) -> String {
    let mut chars: Vec<_> = endstr.chars().collect();
    for op in operations.iter().rev() {
        match *op {
            Op::SwapPositions(x, y) => swap_positions(&mut chars, x, y),
            Op::SwapLetters(x, y) => swap_letters(&mut chars, x, y),
            Op::RotateLeft(x) => rotate_right(&mut chars, x),
            Op::RotateRight(x) => rotate_left(&mut chars, x),
            Op::RotateLetter(x) => unrotate_letter(&mut chars, x),
            Op::Reverse(x, y) => chars[x..=y].reverse(),
            Op::Move(x, y) => move_pos(&mut chars, y, x)
        }
    }
    chars.into_iter().collect()
}

fn part2(operations: &Vec<Op>) -> String {
    unscramble("fbgdceah", operations)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let operations = parse(&input);

    println!("{}", part1(&operations));
    println!("{}", part2(&operations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn test_part1() {
        assert_eq!(&scramble("abcde", &parse(EX)), "decab");
    }

    #[test]
    #[should_panic] // See unrotate_letter() above
    fn test_part2() {
        assert_eq!(&unscramble("decab", &parse(EX)), "abcde");
    }
}
