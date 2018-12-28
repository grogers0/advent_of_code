use std::collections::VecDeque;
use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

fn md5(input: &str) -> String {
    let mut digest = Md5::new();
    digest.input_str(input);
    digest.result_str()
}

// [up, down, left, right]
fn doors_open(passcode: &str, path: &str) -> [bool; 4] {
    fn open(ch: char) -> bool {
        match ch {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false
        }
    }
    let md5 = md5(&format!("{}{}", passcode, path));
    let mut chars_iter = md5.chars();
    [
        open(chars_iter.next().unwrap()),
        open(chars_iter.next().unwrap()),
        open(chars_iter.next().unwrap()),
        open(chars_iter.next().unwrap())
    ]
}

const MAX_X: usize = 3;
const MAX_Y: usize = 3;

fn take_step(input: &str, x: usize, y: usize, path: String, queue: &mut VecDeque<(usize, usize, String)>) {
    let [up, down, left, right] = doors_open(input, &path);
    if x > 0     && up    { queue.push_back((x - 1, y, format!("{}{}", path, 'U'))) }
    if x < MAX_X && down  { queue.push_back((x + 1, y, format!("{}{}", path, 'D'))) }
    if y > 0     && left  { queue.push_back((x, y - 1, format!("{}{}", path, 'L'))) }
    if y < MAX_Y && right { queue.push_back((x, y + 1, format!("{}{}", path, 'R'))) }
}

fn part1(input: &str) -> String {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, "".to_string()));
    while let Some((x, y, path)) = queue.pop_front() {
        if x == MAX_X && y == MAX_Y { return path }
        take_step(input, x, y, path, &mut queue);
    }
    unreachable!()
}

fn part2(input: &str) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, "".to_string()));
    let mut longest_path = 0;
    while let Some((x, y, path)) = queue.pop_front() {
        if x == MAX_X && y == MAX_Y {
            if path.len() > longest_path { longest_path = path.len() }
            continue;
        }
        take_step(input, x, y, path, &mut queue);
    }
    longest_path
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_end();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(&part1("ihgpwlah"), "DDRRRD");
        assert_eq!(&part1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(&part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("ihgpwlah"), 370);
        assert_eq!(part2("kglvqrro"), 492);
        assert_eq!(part2("ulqzkmiv"), 830);
    }
}
