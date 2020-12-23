use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

type Card = u8;
type Deck = VecDeque<Card>;

#[derive(Copy, Clone)]
enum Winner {
    Player1, Player2
}

fn parse(puzzle_input: &str) -> [Deck; 2] {
    fn parse_deck(expected_player: usize, s: &str) -> Deck {
        let mut deck = Deck::new();
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                assert!(line == format!("Player {}:", expected_player));
            } else {
                deck.push_back(line.parse().unwrap());
            }
        }
        deck
    }

    let mut parts = puzzle_input.split("\n\n");
    let deck1 = parse_deck(1, parts.next().unwrap());
    let deck2 = parse_deck(2, parts.next().unwrap());
    assert!(parts.next().is_none());
    for card in deck1.iter() {
        assert!(!deck2.contains(card));
    }
    [deck1, deck2]
}

fn score(deck1: &Deck, deck2: &Deck) -> u64 {
    fn score_deck(deck: &Deck) -> u64 {
        deck.iter().rev().enumerate().map(|(i, c)| (1 + i as u64) * *c as u64).sum()
    }
    // NOTE - loser's score is 0
    score_deck(deck1) + score_deck(deck2)
}

fn part1(puzzle_input: &str) -> u64 {
    let [mut deck1, mut deck2] = parse(puzzle_input);
    loop {
        if deck1.is_empty() || deck2.is_empty() { break }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    score(&deck1, &deck2)
}

fn recursive_combat(deck1: &mut Deck, deck2: &mut Deck) -> Winner {
    let mut seen = HashSet::new();
    loop {
        if deck1.is_empty() {
            return Winner::Player2;
        } else if deck2.is_empty() {
            return Winner::Player1;
        } else if !seen.insert((deck1.clone(), deck2.clone())) {
            return Winner::Player1;
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        let winner = if deck1.len() >= card1 as usize && deck2.len() >= card2 as usize {
            let mut subdeck1 = deck1.iter().take(card1 as usize).cloned().collect();
            let mut subdeck2 = deck2.iter().take(card2 as usize).cloned().collect();
            recursive_combat(&mut subdeck1, &mut subdeck2)
        } else if card1 > card2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player1 => {
                deck1.push_back(card1);
                deck1.push_back(card2);
            },
            Winner::Player2 => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }
    }
}

fn part2(puzzle_input: &str) -> u64 {
    let [mut deck1, mut deck2] = parse(puzzle_input);
    recursive_combat(&mut deck1, &mut deck2);
    score(&deck1, &deck2)
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

    const EX: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() {
        assert_eq!(306, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(291, part2(EX));
    }

    #[test]
    fn test_part2_non_infinite() {
        part2("Player 1:
43
19

Player 2:
2
29
14");
    }
}
