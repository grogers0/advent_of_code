use std::collections::{BTreeSet, VecDeque};
use std::io::{self, Read};

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_points_needed(map: &Vec<Vec<char>>) -> u32 {
    let mut ret = 0;
    for row in map.iter() {
        for ch in row.iter() {
            if let Some(poi) = ch.to_digit(10) {
                ret |= 1 << poi;
            }
        }
    }
    ret
}

fn get_starting_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '0' { return (x, y) }
        }
    }
    panic!()
}

fn move_to(x: usize, y: usize, visited: u32, steps: usize, map: &Vec<Vec<char>>, queue: &mut VecDeque<((usize, usize), u32, usize)>) {
    match map[y][x] {
        '#' => (),
        '.' => queue.push_back(((x, y), visited, steps + 1)),
        ch => {
            let poi = ch.to_digit(10).unwrap();
            queue.push_back(((x, y), visited | (1 << poi), steps + 1));
        }
    }
}

fn steps_to_traverse(input: &str, return_to_start: bool) -> usize {
    let map = parse_map(input);
    let needed = get_points_needed(&map);
    let starting_pos = get_starting_pos(&map);
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((starting_pos, 1, 0));
    while let Some(((x, y), visited, steps)) = queue.pop_front() {
        if visited == needed && (!return_to_start || (x, y) == starting_pos)  { return steps }
        if !seen.insert(((x, y), visited)) { continue }

        move_to(x+1, y, visited, steps, &map, &mut queue);
        move_to(x-1, y, visited, steps, &map, &mut queue);
        move_to(x, y+1, visited, steps, &map, &mut queue);
        move_to(x, y-1, visited, steps, &map, &mut queue);
    }
    panic!()
}

fn part1(input: &str) -> usize {
    steps_to_traverse(input, false)
}

fn part2(input: &str) -> usize {
    steps_to_traverse(input, true)
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
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 14);
    }
}
