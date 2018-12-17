use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

enum TreeVec<T> {
    Leaf(Vec<T>),
    Internal(usize, Vec<Box<TreeVec<T>>>)
}

impl <T> TreeVec<T> {
    fn new() -> TreeVec<T> {
        TreeVec::Leaf(Vec::new())
    }

    fn len(&self) -> usize {
        match self {
            TreeVec::Leaf(elems) => elems.len(),
            TreeVec::Internal(len, _) => *len
        }
    }

    fn remove(&mut self, idx: usize) -> T {
        match self {
            TreeVec::Leaf(elems) => elems.remove(idx),
            TreeVec::Internal(ref mut len, children) => {
                let mut cnt = 0;
                assert!(idx < *len);
                for i in 0..children.len() {
                    if cnt + children[i].len() > idx {
                        *len -= 1;
                        return children[i].remove(idx - cnt);
                    } else {
                        cnt += children[i].len();
                    }
                }
                unreachable!();
            }
        }
    }

    // OK  this is totally messed up but because of the way things are inserted randomly it's good
    // enough for the 100x challenge
    fn insert(&mut self, idx: usize, elem: T) {
        const MAX: usize = 100;
        match self {
            TreeVec::Leaf(elems) => {
                elems.insert(idx, elem);
                if elems.len() == MAX {
                    let elems2 = elems.split_off(MAX/2);
                    let elems1 = elems.split_off(0);
                    *self = TreeVec::Internal(MAX, vec![
                        Box::new(TreeVec::Leaf(elems1)),
                        Box::new(TreeVec::Leaf(elems2))]);
                }
            },
            TreeVec::Internal(ref mut len, children) => {
                let mut cnt = 0;
                assert!(idx < *len);
                for i in 0..children.len() {
                    if cnt + children[i].len() > idx {
                        *len += 1;
                        children[i].insert(idx - cnt, elem);
                        return;
                    } else {
                        cnt += children[i].len();
                    }
                }
                unreachable!();
            }
        }
    }
}

// (players, last marble)
fn parse(input: &str) -> (usize, usize) {
    lazy_static!{
        static ref RE: Regex = Regex::new("^(\\d+) players; last marble is worth (\\d+) points").unwrap();
    }
    let cap = RE.captures(input).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

fn calc(num_players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut player = num_players - 1;
    //let mut circle = Vec::new();
    let mut circle = TreeVec::new();
    circle.insert(0, 0);
    let mut curr_index = 0;
    for marble in 1..=last_marble {
        player = (player + 1) % num_players;

        if marble % 23 == 0 {
            curr_index = (curr_index + circle.len() - 7) % circle.len();
            scores[player] += marble + circle.remove(curr_index);
        } else {
            curr_index = (curr_index + 2) % circle.len();
            circle.insert(curr_index, marble);
        }
    }
    *scores.iter().max().unwrap()
}

fn part1(input: &str) -> usize {
    let (num_players, last_marble) = parse(input);
    calc(num_players, last_marble)
}

fn part2(input: &str) -> usize {
    let (num_players, last_marble) = parse(input);
    calc(num_players, last_marble * 100)
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
        assert_eq!(part1("9 players; last marble is worth 25 points"), 32);
        assert_eq!(part1("10 players; last marble is worth 1618 points"), 8317);
        assert_eq!(part1("13 players; last marble is worth 7999 points"), 146373);
        assert_eq!(part1("17 players; last marble is worth 1104 points"), 2764);
        assert_eq!(part1("21 players; last marble is worth 6111 points"), 54718);
        assert_eq!(part1("30 players; last marble is worth 5807 points"), 37305);
    }
}
