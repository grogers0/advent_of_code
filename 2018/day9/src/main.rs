use std::io::{self, Read};

use lazy_static::lazy_static;
use linked_list::{LinkedList, Cursor};
use regex::Regex;

// (players, last marble)
fn parse(input: &str) -> (usize, usize) {
    lazy_static!{
        static ref RE: Regex = Regex::new("^(\\d+) players; last marble is worth (\\d+) points").unwrap();
    }
    let cap = RE.captures(input).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

fn next_circular(cursor: &mut Cursor<usize>) -> usize {
    if let Some(elem) = cursor.next() {
        *elem
    } else {
        *cursor.next().unwrap()
    }
}

fn prev_circular(cursor: &mut Cursor<usize>) -> usize {
    if let Some(elem) = cursor.prev() {
        *elem
    } else {
        *cursor.prev().unwrap()
    }
}

fn remove_circular(cursor: &mut Cursor<usize>) -> usize {
    if let Some(elem) = cursor.remove() {
        elem
    } else {
        cursor.next();
        cursor.remove().unwrap()
    }
}

fn calc(num_players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut player = num_players - 1;
    let mut circle = LinkedList::new();
    circle.push_back(0);
    let mut cur = circle.cursor();
    for marble in 1..=last_marble {
        player = (player + 1) % num_players;

        if marble % 23 == 0 {
            for _ in 0..7 { prev_circular(&mut cur); }
            scores[player] += marble + remove_circular(&mut cur);
        } else {
            for _ in 0..2 { next_circular(&mut cur); }
            cur.insert(marble);
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
