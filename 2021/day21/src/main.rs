use std::collections::HashMap;
use std::io::{self, Read};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct PlayerState {
    position: usize, // 1-indexed
    score: usize,
}

fn parse(puzzle_input: &str) -> Vec<PlayerState> {
    assert_eq!(2, puzzle_input.lines().count());
    let mut ret = Vec::new();
    for line in puzzle_input.lines() {
        let mut split_iter = line.split("starting position: ");
        split_iter.next().unwrap();
        let position = split_iter.next().unwrap().parse().unwrap();
        assert!(split_iter.next().is_none());
        ret.push(PlayerState { position, score: 0 });
    }
    ret
}

fn part1(puzzle_input: &str) -> u64 {
    let mut die = 0;
    let mut roll_cnt = 0;
    let mut roll = || {
        let ret = die + 1;
        die = ret % 100;
        roll_cnt += 1;
        ret
    };
    let mut players = parse(puzzle_input);
    loop {
        for i in 0..2 {
            let offset = roll() + roll() + roll();
            let position = (players[i].position + offset - 1) % 10 + 1;
            let score = players[i].score + position;
            players[i].position = position;
            players[i].score = score;

            if score >= 1000 {
                return players[(i + 1) % 2].score as u64 * roll_cnt;
            }
        }
    }
}

fn part2(puzzle_input: &str) -> u64 {
    // Returns how many universes each player won in
    fn count_outcomes(players: Vec<PlayerState>,
        memo: &mut HashMap<Vec<PlayerState>, Vec<u64>>) -> Vec<u64>
    {
        if players.iter().any(|p| p.score >= 21) {
            return players.iter()
                .map(|p| if p.score >= 21 { 1 } else { 0 })
                .collect();
        } else if let Some(ret) = memo.get(&players) {
            return ret.clone();
        }

        let mut ret = vec![0u64; 2];
        const ROLL_OUTCOMES: [u64; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
        for roll in 3..ROLL_OUTCOMES.len() {
            let position = (players[0].position + roll - 1) % 10 + 1;
            let score = players[0].score + position;
            let players = vec![players[1], PlayerState { position, score }];
            let outcomes = count_outcomes(players, memo);
            ret[0] += ROLL_OUTCOMES[roll] * outcomes[1];
            ret[1] += ROLL_OUTCOMES[roll] * outcomes[0];
        }
        memo.insert(players, ret.clone());
        ret
    }

    let players = parse(puzzle_input);
    let outcomes = count_outcomes(players, &mut HashMap::new());
    *outcomes.iter().max().unwrap()
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

    const EX: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part1() {
        assert_eq!(739785, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(444356092776315, part2(EX));
    }
}
