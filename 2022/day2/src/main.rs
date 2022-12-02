use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw
}

fn outcome_of_round(opponent: Shape, player: Shape) -> Outcome {
    match opponent {
        Shape::Rock => match player {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Loss,
        },
        Shape::Paper => match player {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Win,
        },
        Shape::Scissors => match player {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Loss,
            Shape::Scissors => Outcome::Draw,
        },
    }
}

fn required_play(opponent: Shape, outcome: Outcome) -> Shape {
    match opponent {
        Shape::Rock => match outcome {
            Outcome::Draw => Shape::Rock,
            Outcome::Win => Shape::Paper,
            Outcome::Loss => Shape::Scissors,
        },
        Shape::Paper => match outcome {
            Outcome::Loss => Shape::Rock,
            Outcome::Draw => Shape::Paper,
            Outcome::Win => Shape::Scissors,
        },
        Shape::Scissors => match outcome {
            Outcome::Win => Shape::Rock,
            Outcome::Loss => Shape::Paper,
            Outcome::Draw => Shape::Scissors,
        },
    }
}

fn score(outcome: Outcome, player: Shape) -> u64 {
    (match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }) + (match player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    })
}

fn score_part1(opponent: Shape, player: Shape) -> u64 {
    score(outcome_of_round(opponent, player), player)
}

fn score_part2(opponent: Shape, outcome: Outcome) -> u64 {
    score(outcome, required_play(opponent, outcome))
}

fn parse_opponent(s: &str) -> Shape {
    match s {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!(),
    }
}

fn parse_part1(puzzle_input: &str) -> Vec<(Shape, Shape)> {
    puzzle_input.trim_end().split("\n")
        .map(|line| {
            let mut sp = line.split(" ");
            let opponent = parse_opponent(sp.next().unwrap());
            let player = match sp.next().unwrap() {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!(),
            };
            assert!(sp.next().is_none());
            (opponent, player)
        })
        .collect()
}

fn parse_part2(puzzle_input: &str) -> Vec<(Shape, Outcome)> {
    puzzle_input.trim_end().split("\n")
        .map(|line| {
            let mut sp = line.split(" ");
            let opponent = parse_opponent(sp.next().unwrap());
            let outcome = match sp.next().unwrap() {
                "X" => Outcome::Loss,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!(),
            };
            assert!(sp.next().is_none());
            (opponent, outcome)
        })
        .collect()
}

fn part1(puzzle_input: &str) -> u64 {
    let strats = parse_part1(puzzle_input);
    strats.iter().map(|&(opponent, player)| score_part1(opponent, player)).sum()
}


fn part2(puzzle_input: &str) -> u64 {
    let strats = parse_part2(puzzle_input);
    strats.iter().map(|&(opponent, outcome)| score_part2(opponent, outcome)).sum()
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

    const EX: &str = "A Y
B X
C Z
";


    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 12);
    }
}
