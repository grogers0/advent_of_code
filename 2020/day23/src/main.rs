use std::io::{self, Read};

fn parse(puzzle_input: &str) -> (Vec<usize>, usize) {
    let ord_cups: Vec<usize> = puzzle_input.trim_end().chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize - 1).collect();
    for cup in 0..ord_cups.len() {
        debug_assert!(ord_cups.contains(&cup));
    }
    let mut cups = vec![0usize; ord_cups.len()];
    for i in 1..ord_cups.len() {
        cups[ord_cups[i-1]] = ord_cups[i];
    }
    cups[ord_cups[ord_cups.len()-1]] = ord_cups[0];
    (cups, ord_cups[0])
}

fn cycle(cups: &mut [usize], curr: usize) -> usize {
    let mut picked_cups = [0usize; 3];
    let mut next = cups[curr];
    for i in 0..3 {
        picked_cups[i] = next;
        next = cups[next];
    }
    cups[curr] = next;

    let mut dest = curr;
    loop {
        if dest == 0 { dest = cups.len() }
        dest -= 1;
        if !picked_cups.contains(&dest) { break }
    }
    cups[picked_cups[2]] = cups[dest];
    cups[dest] = picked_cups[0];
    next
}

fn part1(puzzle_input: &str) -> String {
    let (mut cups, mut curr) = parse(puzzle_input);
    for _ in 0..100 {
        curr = cycle(&mut cups, curr);
    }
    let mut ret = String::new();
    let mut next = cups[0];
    while next != 0 {
        ret.push_str(&format!("{}", next + 1));
        next = cups[next];
    }
    ret
}

fn part2(puzzle_input: &str) -> u64 {
    let (mut cups, first) = parse(puzzle_input);
    let initial_len = cups.len();
    let last = cups.iter().position(|&v| v == first).unwrap();
    cups.resize(1_000_000, usize::MAX);
    cups[last] = initial_len;
    for i in initial_len+1..cups.len() {
        cups[i-1] = i;
    }
    cups[999_999] = first;

    let mut curr = first;
    for _ in 0..10_000_000 {
        curr = cycle(&mut cups, curr);
    }
    let v1 = cups[0];
    let v2 = cups[v1];
    (v1 as u64 + 1) * (v2 as u64 + 1)
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

    #[test]
    fn test_part1() {
        assert_eq!("67384529".to_string(), part1("389125467"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(149245887792, part2("389125467"));
    }
}
