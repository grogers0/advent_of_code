use std::io::{self, Read};

fn skip_garbage(input: &mut Iterator<Item=char>) {
    let mut bang = false;
    loop {
        if bang {
            input.next();
            bang = false;
            continue;
        }
        match input.next() {
            Some('!') => bang = true,
            Some('>') => return,
            Some(_) => (),
            None => unreachable!()
        }
    }
}

fn sum_groups(input: &mut Iterator<Item=char>, indent: usize) -> usize {
    let mut sum = 0;
    loop {
        match input.next() {
            Some('{') => sum += sum_groups(input, indent+1),
            Some('<') => skip_garbage(input), 
            Some(',') => (),
            Some('}') => return sum + indent,
            Some(_) => unreachable!(),
            None => return sum
        }
    }
}

fn part1(input: &str) -> usize {
    sum_groups(&mut input.trim_end().chars(), 0)
}

fn part2(input: &str) -> usize {
    let mut bang = false;
    let mut garbage = false;
    let mut count = 0;
    for ch in input.trim_end().chars() {
        if bang {
            bang = false;
            continue;
        }
        match ch {
            '<' if !garbage => garbage = true,
            '!' => bang = true,
            '>' => garbage = false,
            _ => if garbage { count += 1 }
        }
    }
    count
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
        assert_eq!(part1("{}"), 1);
        assert_eq!(part1("{{{}}}"), 6);
        assert_eq!(part1("{{},{}}"), 5);
        assert_eq!(part1("{{{},{},{{}}}}"), 16);
        assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("<>"), 0);
        assert_eq!(part2("<random characters>"), 17);
        assert_eq!(part2("<<<<>"), 3);
        assert_eq!(part2("<{!>}>"), 2);
        assert_eq!(part2("<!!>"), 0);
        assert_eq!(part2("<!!!>>"), 0);
        assert_eq!(part2("<{o\"i!a,<{i<a>"), 10);
    }
}
