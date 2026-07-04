use std::io::{self, Read};
use std::collections::HashMap;
use std::iter;

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
fn numpad_y(ch: char) -> usize {
    match ch {
        '7' | '8' | '9' => 0,
        '4' | '5' | '6' => 1,
        '1' | '2' | '3' => 2,
        '0' | 'A'       => 3,
        _ => panic!(),
    }
}
fn numpad_x(ch: char) -> usize {
    match ch {
        '1' | '4' | '7'       => 0,
        '0' | '2' | '5' | '8' => 1,
        'A' | '3' | '6' | '9' => 2,
        _ => panic!(),
    }
}
fn numpad_step(a: char, b: char) -> String {
    let ax = numpad_x(a);
    let ay = numpad_y(a);
    let bx = numpad_x(b);
    let by = numpad_y(b);
    // This is the optimal order based on moving the robots back and forth to the A key, unless the
    // hole gets in the way...
    let mut order = ['<', 'v', '^', '>'];
    if (ax == 0 && by == 3) || (bx == 0 && ay == 3) {
        order = ['^', '>', 'v', '<'];
    }
    let mut ret = String::new();
    for ch in order {
        match ch {
            '>' => if ax < bx { ret.extend(iter::repeat('>').take(bx - ax)); },
            '^' => if ay > by { ret.extend(iter::repeat('^').take(ay - by)); },
            '<' => if ax > bx { ret.extend(iter::repeat('<').take(ax - bx)); },
            'v' => if ay < by { ret.extend(iter::repeat('v').take(by - ay)); },
            _ => panic!(),
        };
    }
    ret.push('A');
    ret
}
fn numpad_path(input: &str) -> String {
    let mut ret = String::new();
    let mut curr = 'A';
    for next in input.chars() {
        ret.extend(numpad_step(curr, next).chars());
        curr = next;
    }
    ret
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn dpad_y(ch: char) -> usize {
    match ch {
        '^' | 'A'       => 0,
        '<' | 'v' | '>' => 1,
        _ => panic!(),
    }
}
fn dpad_x(ch: char) -> usize {
    match ch {
        '<'       => 0,
        '^' | 'v' => 1,
        'A' | '>' => 2,
        _ => panic!(),
    }
}
fn dpad_step(a: char, b: char) -> String {
    let ax = dpad_x(a);
    let ay = dpad_y(a);
    let bx = dpad_x(b);
    let by = dpad_y(b);
    // This is the optimal order based on moving the robots back and forth to the A key, unless the
    // hole gets in the way...
    let mut order = ['<', 'v', '^', '>'];
    if (ax == 0 && by == 0) || (bx == 0 && ay == 0) {
        order = ['>', '^', 'v', '<'];
    }
    let mut ret = String::new();
    for ch in order {
        match ch {
            '>' => if ax < bx { ret.extend(iter::repeat('>').take(bx - ax)); },
            '^' => if ay > by { ret.extend(iter::repeat('^').take(ay - by)); },
            '<' => if ax > bx { ret.extend(iter::repeat('<').take(ax - bx)); },
            'v' => if ay < by { ret.extend(iter::repeat('v').take(by - ay)); },
            _ => panic!(),
        };
    }
    ret.push('A');
    ret
}

fn find_cost(input: &str, num_dpads: usize, memo: &mut HashMap<(String, usize), usize>) -> usize {
    if num_dpads == 0 { return input.len(); }
    if let Some(ret) = memo.get(&(input.to_string(), num_dpads)) { return *ret; }
    let mut sum = 0;
    let mut curr = 'A';
    for next in input.chars() {
        let step = dpad_step(curr, next);
        sum += find_cost(&step, num_dpads - 1, memo);
        curr = next;
    }
    memo.insert((input.to_string(), num_dpads), sum);
    sum
}

fn find_total_complexities(num_dpads: usize, puzzle_input: &str) -> usize {
    let mut sum = 0;
    for line in puzzle_input.lines() {
        let mut memo = HashMap::new();
        assert_eq!(&line[(line.len()-1)..line.len()], "A");
        let num = line[0..(line.len()-1)].parse::<usize>().unwrap();
        let path = numpad_path(line);
        sum += num * find_cost(&path, num_dpads, &mut memo);
    }
    sum
}

fn part1(puzzle_input: &str) -> usize {
    find_total_complexities(2, puzzle_input)
}

fn part2(puzzle_input: &str) -> usize {
    find_total_complexities(25, puzzle_input)
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

    const EX: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 126384);
    }
}
