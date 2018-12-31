use std::io::{self, Read};

fn step(floor: &mut i64, ch: char) {
    match ch {
        '(' => *floor += 1,
        ')' => *floor -= 1,
        _ => panic!()
    }
}

fn part1(input: &str) -> i64 {
    let mut floor = 0;
    for ch in input.trim_end().chars() {
        step(&mut floor, ch);
    }
    floor
}

fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (i, ch) in input.trim_end().chars().enumerate() {
        step(&mut floor, ch);
        if floor == -1 { return i + 1; }
    }
    panic!()
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
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
