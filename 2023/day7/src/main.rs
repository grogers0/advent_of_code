use std::io::{self, Read};

const JOKER: u8 = 9;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Card(u8);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Hand([Card; 5]);


struct HandWithBid {
    hand: Hand,
    bid: u64,
}

impl Card {
    fn parse(ch: char) -> Card {
        Card(match ch {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '2'..='9' => (ch as u8) - ('2' as u8),
            _ => panic!(),
        })
    }
}

fn get_hand_type_part1(hand: &Hand) -> HandType {
    let mut by_val = [0usize; 13];
    for card in &hand.0 {
        by_val[card.0 as usize] += 1;
    }
    let mut num_pairs = 0;
    let mut has_triple = false;
    for cnt in by_val {
        if cnt == 5 {
            return HandType::FiveOfAKind;
        } else if cnt == 4 {
            return HandType::FourOfAKind;
        } else if cnt == 3 {
            has_triple = true;
        } else if cnt == 2 {
            num_pairs += 1;
        }
    }

    if has_triple {
        if num_pairs == 1 {
            HandType::FullHouse
        } else {
            debug_assert_eq!(num_pairs, 0);
            HandType::ThreeOfAKind
        }
    } else if num_pairs == 2 {
        HandType::TwoPair
    } else if num_pairs == 1 {
        HandType::OnePair
    } else {
        assert_eq!(num_pairs, 0);
        HandType::HighCard
    }
}

fn get_hand_type_part2(hand: &Hand) -> HandType {
    let mut by_val = [0usize; 13];
    for card in &hand.0 {
        by_val[card.0 as usize] += 1;
    }
    let mut num_jokers = by_val[JOKER as usize];
    by_val[JOKER as usize] = 0;
    let mut num_pairs = 0;
    let mut has_triple = false;
    for cnt in by_val {
        if cnt + num_jokers == 5 {
            return HandType::FiveOfAKind;
        }
    }
    for cnt in by_val {
        if cnt + num_jokers == 4 {
            return HandType::FourOfAKind;
        }
    }
    for cnt in &mut by_val {
        if *cnt + num_jokers == 3 {
            has_triple = true;
            *cnt += num_jokers;
            num_jokers = 0;
        }
    }
    for cnt in &mut by_val {
        if *cnt + num_jokers == 2 {
            num_pairs += 1;
            *cnt += num_jokers;
            num_jokers = 0;
        }
    }

    if has_triple {
        if num_pairs == 1 {
            HandType::FullHouse
        } else {
            debug_assert_eq!(num_pairs, 0);
            HandType::ThreeOfAKind
        }
    } else if num_pairs == 2 {
        HandType::TwoPair
    } else if num_pairs == 1 {
        HandType::OnePair
    } else {
        assert_eq!(num_pairs, 0);
        HandType::HighCard
    }
}

fn parse_hand(s: &str) -> Hand {
    let mut cards = [Card(255); 5];
    let mut chars = s.chars();
    for i in 0..5 {
        cards[i] = Card::parse(chars.next().unwrap());
    }
    assert!(chars.next().is_none());
    Hand(cards)
}

fn parse(puzzle_input: &str) -> Vec<HandWithBid> {
    puzzle_input.lines().map(|line| {
        let mut sp_iter = line.split_whitespace();
        let hand = parse_hand(sp_iter.next().unwrap());
        let bid = sp_iter.next().unwrap().parse().unwrap();
        assert!(sp_iter.next().is_none());
        HandWithBid { hand, bid }
    }).collect()
}

fn hand_number_part1(hand: &Hand) -> u64 {
    let mut ret = 0;
    for card in &hand.0 {
        ret = ret * 13 + card.0 as u64;
    }
    ret
}

fn hand_number_part2(hand: &Hand) -> u64 {
    let mut ret = 0;
    for card in &hand.0 {
        let num = if card.0 == JOKER {
            0
        } else if card.0 < JOKER {
            card.0 as u64 + 1
        } else {
            card.0 as u64
        };
        ret = ret * 13 + num;
    }
    ret
}

fn part1(hands_with_bids: &[HandWithBid]) -> u64 {
    let mut sorted_hands: Vec<_> =
        hands_with_bids.iter()
        .map(|hand_with_bid|
            (
                get_hand_type_part1(&hand_with_bid.hand),
                hand_number_part1(&hand_with_bid.hand),
                hand_with_bid.bid)
            )
        .collect();
    sorted_hands.sort_unstable();
    let mut sum = 0;
    for (i, (_hand_type, _hand, bid)) in sorted_hands.iter().enumerate() {
        sum += (i as u64 + 1) * bid;
    }
    sum
}

fn part2(hands_with_bids: &[HandWithBid]) -> u64 {
    let mut sorted_hands: Vec<_> =
        hands_with_bids.iter()
        .map(|hand_with_bid|
            (
                get_hand_type_part2(&hand_with_bid.hand),
                hand_number_part2(&hand_with_bid.hand),
                hand_with_bid.bid)
            )
        .collect();
    sorted_hands.sort_unstable();
    let mut sum = 0;
    for (i, (_hand_type, _hand, bid)) in sorted_hands.iter().enumerate() {
        sum += (i as u64 + 1) * bid;
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let hands_with_bids = parse(&puzzle_input);
    println!("{}", part1(&hands_with_bids));
    println!("{}", part2(&hands_with_bids));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 5905);
    }
}
