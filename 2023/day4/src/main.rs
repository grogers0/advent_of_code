use std::collections::HashSet;
use std::io::{self, Read};

struct Card {
    winning_nums: HashSet<u32>,
    contains_nums: Vec<u32>,
}

fn parse(puzzle_input: &str) -> Vec<usize> {
    let raw_cards = puzzle_input.lines().map(|line| {
        let offset = line.find(':').unwrap() + 1;
        let line = &line[offset..];
        let mut winning_nums = HashSet::new();
        let mut contains_nums = Vec::new();
        let mut parsing_winning = true;
        for token in line.split_whitespace() {
            if token == "|" {
                debug_assert!(parsing_winning);
                parsing_winning = false;
            } else {
                let num = token.parse().unwrap();
                if parsing_winning {
                    winning_nums.insert(num);
                } else {
                    contains_nums.push(num);
                }
            }
        }
        Card { winning_nums, contains_nums }
    }).collect::<Vec<_>>();

    let mut scored_cards = Vec::with_capacity(raw_cards.len());
    for card in raw_cards {
        let mut cnt_winning = 0;
        for num in &card.contains_nums {
            if card.winning_nums.contains(&num) {
                cnt_winning += 1;
            }
        }
        scored_cards.push(cnt_winning);
    }

    scored_cards
}

fn part1(scored_cards: &[usize]) -> usize {
    scored_cards.iter()
        .filter(|&num_winning| *num_winning > 0)
        .map(|num_winning| 1 << (num_winning - 1))
        .sum()
}

fn part2(scored_cards: &[usize]) -> usize {
    // Original cards
    let mut card_counts = vec![1; scored_cards.len()];
    for i in 0..scored_cards.len() {
        debug_assert!(i + scored_cards[i] < scored_cards.len());
        for j in (i + 1)..(i + 1 + scored_cards[i]) {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.iter().cloned().sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let scored_cards = parse(&puzzle_input);
    println!("{}", part1(&scored_cards));
    println!("{}", part2(&scored_cards));
}

#[cfg(test)]
mod tests {
    use super::*;

const EX: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 30);
    }
}
