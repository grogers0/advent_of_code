use std::collections::{BTreeSet, VecDeque};
use std::io::{self, Read};

fn is_wall(x: usize, y: usize, fav_num: usize) -> bool {
    let v = x*x + 3*x + 2*x*y + y + y*y + fav_num;
    v.count_ones() % 2 == 1
}

fn step_adjacent(x: usize, y: usize, dist: usize, queue: &mut VecDeque<(usize, usize, usize)>) {
    if x > 0 { queue.push_back((x-1, y, dist+1)); }
    if y > 0 { queue.push_back((x, y-1, dist+1)); }
    queue.push_back((x+1, y, dist+1));
    queue.push_back((x, y+1, dist+1));
}

fn distance_to(target_x: usize, target_y: usize, fav_num: usize) -> usize {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 1, 0)); // Starting point
    while let Some((x, y, dist)) = queue.pop_front() {
        if x == target_x && y == target_y { return dist }
        if is_wall(x, y, fav_num) { continue }
        if !seen.insert((x, y)) { continue }

        step_adjacent(x, y, dist, &mut queue);
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    let fav_num = input.trim_end().parse().unwrap();
    distance_to(31, 39, fav_num)
}

fn part2(input: &str) -> usize {
    let fav_num = input.trim_end().parse().unwrap();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((1, 1, 0)); // Starting point
    let mut cnt = 0;
    while let Some((x, y, dist)) = queue.pop_front() {
        if dist > 50 { break }
        if is_wall(x, y, fav_num) { continue }
        if !seen.insert((x, y)) { continue }

        cnt += 1;
        step_adjacent(x, y, dist, &mut queue);
    }
    cnt
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
        assert_eq!(distance_to(7, 4, 10), 11);
    }
}
