use std::collections::HashSet;
use std::io::{self, Read};

struct Map {
    width: usize,
    height: usize,
    galaxies: HashSet<(usize, usize)>
}

fn parse(puzzle_input: &str) -> Map {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut galaxies = HashSet::new();
    for (y, line) in puzzle_input.lines().enumerate() {
        assert_eq!(width, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    galaxies.insert((x, y));
                },
                '.' => (),
                _ => panic!(),
            };
        }
    }
    Map { width, height, galaxies }
}

fn expand(map: &Map, multiplier: usize) -> Map {
    let mut galaxies = HashSet::with_capacity(map.galaxies.len());
    let mut empty_cols = vec![false; map.width];
    let mut empty_rows = vec![false; map.height];
    let mut extra_cols = 0;
    let mut extra_rows = 0;
    for y in 0..map.height {
        if (0..map.width).all(|x| !map.galaxies.contains(&(x, y))) {
            extra_rows += 1;
            empty_rows[y] = true;
        }
    }
    for x in 0..map.width {
        if (0..map.height).all(|y| !map.galaxies.contains(&(x, y))) {
            extra_cols += 1;
            empty_cols[x] = true;
        }
    }
    let width = map.width + extra_cols * (multiplier - 1);
    let height = map.height + extra_rows * (multiplier - 1);
    for &(x, y) in &map.galaxies {
        let x = x + (0..x).filter(|&x| empty_cols[x]).count() * (multiplier - 1);
        let y = y + (0..y).filter(|&y| empty_rows[y]).count() * (multiplier - 1);
        galaxies.insert((x, y));
    }
    Map { width, height, galaxies }
}

fn manhattan_dist(x1: usize, y1: usize, x2: usize, y2: usize) -> u64 {
    let mut sum = 0;
    sum += if x1 > x2 { x1 - x2 } else { x2 - x1 };
    sum += if y1 > y2 { y1 - y2 } else { y2 - y1 };
    sum as u64
}

fn sum_all_pairwise_distances(map: &Map) -> u64 {
    let galaxies: Vec<_> = map.galaxies.iter().cloned().collect();
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            sum += manhattan_dist(
                galaxies[i].0, galaxies[i].1,
                galaxies[j].0, galaxies[j].1);
        }
    }
    sum
}

fn part1(map: &Map) -> u64 {
    sum_all_pairwise_distances(&expand(map, 2))
}

fn part2(map: &Map) -> u64 {
    sum_all_pairwise_distances(&expand(map, 1_000_000))
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let map = parse(&puzzle_input);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            sum_all_pairwise_distances(&expand(&parse(EX), 10)),
            1030);
        assert_eq!(
            sum_all_pairwise_distances(&expand(&parse(EX), 100)),
            8410);
    }
}
