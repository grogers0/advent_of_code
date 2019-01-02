use std::io::{self, Read};

fn parse_map(input: &str) -> Vec<Vec<bool>> {
    let map: Vec<Vec<bool>> = input.lines().map(|line| {
        line.chars().map(|ch| {
            match ch {
                '#' => true,
                '.' => false,
                _ => panic!()
            }
        }).collect()
    }).collect();
    let height = map.len();
    for row in map.iter() {
        assert_eq!(row.len(), height); // Should be square
    }
    map
}

fn neighbors_on(x: usize, y: usize, map: &Vec<Vec<bool>>) -> usize {
    let len = map.len();
    let mut cnt = 0;
    if x > 0 {
        if y > 0 && map[y-1][x-1] { cnt += 1 }
        if map[y][x-1] { cnt += 1 }
        if y < len-1 && map[y+1][x-1] { cnt += 1 }
    }
    if y > 0 && map[y-1][x] { cnt += 1 }
    if y < len-1 && map[y+1][x] { cnt += 1 }
    if x < len-1 {
        if y > 0 && map[y-1][x+1] { cnt += 1 }
        if map[y][x+1] { cnt += 1 }
        if y < len-1 && map[y+1][x+1] { cnt += 1 }
    }
    cnt
}

fn next_map(curr_map: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let len = curr_map.len();
    let mut map = vec![vec![false; len]; len];
    for y in 0..len {
        for x in 0..len {
            map[y][x] = if curr_map[y][x] {
                let neighbors = neighbors_on(x, y, &curr_map);
                neighbors == 2 || neighbors == 3
            } else {
                neighbors_on(x, y, &curr_map) == 3
            }
        }
    }
    map
}

fn set_corners_on(map: &mut Vec<Vec<bool>>) {
    let len = map.len();
    map[0][0] = true;
    map[0][len-1] = true;
    map[len-1][0] = true;
    map[len-1][len-1] = true;
}

fn count_lights_after_steps(input: &str, steps: usize, corners_on: bool) -> usize {
    let mut map = parse_map(input);
    if corners_on { set_corners_on(&mut map) }
    for _ in 0..steps {
        map = next_map(map);
        if corners_on { set_corners_on(&mut map) }
    }

    map.iter().map(|row| row.iter().filter(|light| **light).count()).sum()
}

fn part1(input: &str) -> usize {
    count_lights_after_steps(input, 100, false)
}

fn part2(input: &str) -> usize {
    count_lights_after_steps(input, 100, true)
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

    const EX: &str = "\
.#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn test_part1() {
        assert_eq!(count_lights_after_steps(EX, 4, false), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_lights_after_steps(EX, 5, true), 17);
    }
}
