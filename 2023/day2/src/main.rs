use std::cmp::max;
use std::collections::HashMap;
use std::io::{self, Read};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Color {
    RED,
    GREEN,
    BLUE
}

impl Color {
    fn from_str(s: &str) -> Color {
        match s {
            "red" => Color::RED,
            "green" => Color::GREEN,
            "blue" => Color::BLUE,
            _ => panic!()
        }
    }
}

type Game = Vec<HashMap<Color, usize>>;

fn parse(puzzle_input: &str) -> Vec<Game> {
    let regex = Regex::new(" ([0-9]+) (red|green|blue)").unwrap();
    puzzle_input.lines().map(|line| {
        let line = &line[line.find(':').unwrap() + 1 ..];
        line.split(";").map(|set_str| {
            set_str.split(",").map(|cube_str| {
                let cap = regex.captures(cube_str).unwrap();
                let num = cap[1].parse().unwrap();
                let color = Color::from_str(&cap[2]);
                (color, num)
            }).collect()
        }).collect()
    }).collect()
}

fn part1(games: &[Game]) -> usize {
    let mut sum = 0;
    let mut max_cubes = HashMap::new();
    max_cubes.insert(Color::RED, 12);
    max_cubes.insert(Color::GREEN, 13);
    max_cubes.insert(Color::BLUE, 14);
    let max_cubes = max_cubes;

    for (i, game) in games.iter().enumerate() {
        let mut possible = true;
        'set: for set in game {
            for (color, num) in set {
                if num > &max_cubes[color] {
                    possible = false;
                    break 'set;
                }
            }
        }
        if possible {
            sum += i + 1;
        }
    }
    sum
}

fn part2(games: &[Game]) -> usize {
    let mut sum = 0;
    for game in games {
        let mut max_cubes = HashMap::new();
        for set in game {
            for (color, num) in set {
                max_cubes.entry(color)
                    .and_modify(|max_num| *max_num = max(*max_num, *num))
                    .or_insert(*num);
            }
        }
        let mut pow = 1;
        for (_, num) in max_cubes {
            pow *= num;
        }
        sum += pow;
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let games = parse(&puzzle_input);
    println!("{}", part1(&games));
    println!("{}", part2(&games));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 2286);
    }
}
