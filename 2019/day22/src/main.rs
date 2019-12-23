use std::fmt::Display;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

type Card = u16;
type Deck = Vec<Card>;

lazy_static! {
    static ref CUT_RE: Regex = Regex::new("cut (-?[0-9]+)").unwrap();
    static ref DEAL_WITH_INCRREMENT_RE: Regex =
        Regex::new("deal with increment ([0-9]+)").unwrap();
    static ref DEAL_INTO_NEW_STACK_RE: Regex =
        Regex::new("deal into new stack").unwrap();
}

fn new_deck(n: usize) -> Deck {
    (0 .. n as Card).collect()
}

fn deal_into_new_stack(mut deck: Deck) -> Deck {
    deck.reverse();
    deck
}

fn cut(n: isize, mut deck: Deck) -> Deck {
    assert!((n.abs() as usize) < deck.len());
    let n = if n >= 0 { n as usize } else { deck.len() - n.abs() as usize };
    let mut prefix = deck.drain(0 .. n).collect();
    deck.append(&mut prefix);
    deck
}

fn deal_with_increment(n: usize, deck: Deck) -> Deck {
    assert!(n <= deck.len());
    let deck_len = deck.len();
    let mut deck2 = vec![0; deck_len];
    let mut offset = 0;
    for card in deck {
        deck2[offset] = card;
        offset = (offset + n) % deck_len;
    }
    deck2
}

fn parse_part1(input: &str) -> Vec<Box<dyn Fn(Deck) -> Deck>> {
    input.trim().lines().map(|line| {
        let technique: Box<dyn Fn(Deck) -> Deck> = if let Some(cap) = CUT_RE.captures(line) {
            let offset = cap[1].parse().unwrap();
            Box::new(move |deck| cut(offset, deck))
        } else if let Some(cap) = DEAL_WITH_INCRREMENT_RE.captures(line) {
            let offset = cap[1].parse().unwrap();
            Box::new(move |deck| deal_with_increment(offset, deck))
        } else if DEAL_INTO_NEW_STACK_RE.is_match(line) {
            Box::new(|deck| deal_into_new_stack(deck))
        } else {
            panic!()
        };
        technique
    }).collect()
}

fn shuffle(techniques: Vec<Box<dyn Fn(Deck) -> Deck>>, mut deck: Deck) -> Deck {
    for technique in techniques {
        deck = technique(deck);
    }
    deck
}

fn part1(input: &str) -> impl Display {
    let deck = shuffle(parse_part1(input), new_deck(10007));
    for (i, card) in deck.into_iter().enumerate() {
        if card == 2019 { return i }
    }
    panic!()
}

// I could not figure this out on my own. For more details on what this is doing, see:
// https://przybyl.io/solution-explanation-to-day-22-of-advent-of-code-2019.html
// I don't pretend to understand it well.
fn part2(input: &str) -> impl Display {
    // a mod m
    fn modulo(a: i128, m: i128) -> i128 {
        ((a % m) + m) % m
    }
    // a^-1 mod m
    fn modinv(a: i128, m: i128) -> i128 {
        let mut mn = (m, a);
        let mut xy = (0, 1);
        while mn.1 != 0 {
            xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
            mn = (mn.1, mn.0 % mn.1);
        }
        while xy.0 < 0 { xy.0 += m };
        xy.0
    }
    // a^b mod m
    fn modpow(mut a: i128, mut b: i128, m: i128) -> i128 {
        let mut y = 1;
        while b > 1 {
            if b % 2 == 0 {
                a = modulo(a * a, m);
                b = b / 2;
            } else {
                y = modulo(a * y, m);
                a = modulo(a * a, m);
                b = (b - 1) / 2;
            }
        }
        modulo(a * y, m)
    }
    let n = 119315717514047;
    let reps = 101741582076661;
    let mut f = (1, 0);
    for line in input.trim().lines().rev() {
        if let Some(cap) = CUT_RE.captures(line) {
            let offset = modulo(cap[1].parse().unwrap(), n);
            f = (modulo(f.0, n), modulo(f.1 + offset, n));
        } else if let Some(cap) = DEAL_WITH_INCRREMENT_RE.captures(line) {
            let offset = modinv(modulo(cap[1].parse().unwrap(), n), n);
            f = (modulo(offset * f.0, n), modulo(offset * f.1, n));
        } else if DEAL_INTO_NEW_STACK_RE.is_match(line) {
            f = (modulo(-f.0, n), modulo(-f.1 - 1 + n, n));
        } else {
            panic!()
        };
    }

    let ak = modpow(f.0, reps, n);
    let geo = if f.0 != 1 { modulo((ak - 1) * modinv(f.0 - 1, n), n) } else { reps };
    let pos = 2020;
    modulo(ak * pos + geo * f.1, n)
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
    fn test_part1_deal_into_new_stack() {
        assert_eq!(deal_into_new_stack(new_deck(10)),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_part1_cut() {
        assert_eq!(cut(3, new_deck(10)),
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        assert_eq!(cut(-4, new_deck(10)),
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_part1_deal_with_increment() {
        assert_eq!(deal_with_increment(3, new_deck(10)),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_part1_ex1() {
        let ex = "
deal with increment 7
deal into new stack
deal into new stack";
        assert_eq!(shuffle(parse_part1(ex), new_deck(10)),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_part1_ex2() {
        let ex = "
cut 6
deal with increment 7
deal into new stack";
        assert_eq!(shuffle(parse_part1(ex), new_deck(10)),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_part1_ex3() {
        let ex = "
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        assert_eq!(shuffle(parse_part1(ex), new_deck(10)),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
