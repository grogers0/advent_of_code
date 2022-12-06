use std::io::{self, Read};
use std::collections::{VecDeque, HashSet};

fn is_unique(chunk: &VecDeque<char>) -> bool {
    let mut set = HashSet::new();
    for ch in chunk {
        set.insert(ch);
    }
    set.len() == chunk.len()
}

fn search_distinct_sequence(data: &str, num_distinct: usize) -> usize {
    let mut chunk = VecDeque::with_capacity(num_distinct);
    for (i, ch) in data.chars().enumerate() {
        if chunk.len() == num_distinct {
            chunk.pop_front();
        }
        chunk.push_back(ch);
        if chunk.len() == num_distinct && is_unique(&chunk) {
            return i + 1;
        }
    }
    panic!()
}

fn part1(puzzle_input: &str) -> usize {
    search_distinct_sequence(puzzle_input, 4)
}

fn part2(puzzle_input: &str) -> usize {
    search_distinct_sequence(puzzle_input, 14)
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
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
