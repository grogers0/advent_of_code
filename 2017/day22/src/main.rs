use std::collections::{BTreeSet, BTreeMap};
use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
enum Status {
    //Clean, // implied by non-existence in the grid
    Weakened,
    Infected,
    Flagged
}

fn parse(input: &str) -> BTreeSet<(isize,isize)> {
    let size = input.lines().next().unwrap().chars().count() as isize;
    assert_eq!(size % 2, 1);
    let bound = (size - 1) / 2;
    let mut infected = BTreeSet::new();

    let mut y = -bound;
    for line in input.lines() {
        let mut x = -bound;
        for ch in line.chars() {
            match ch {
                '#' => { infected.insert((x, y)); },
                '.' => (),
                _ => unreachable!()
            }
            x += 1;
        }
        y += 1;
    }
    infected
}

fn turn_left(dir: &mut (isize, isize)) {
    if dir.0 == 0 { // vertical
        *dir = (dir.1, 0)
    } else { // horizontal
        *dir = (0, -dir.0)
    }
}

fn turn_right(dir: &mut (isize, isize)) {
    if dir.0 == 0 { // vertical
        *dir = (-dir.1, 0)
    } else { // horizontal
        *dir = (0, dir.0)
    }
}

fn turn_around(dir: &mut (isize, isize)) {
    *dir = (-dir.0, -dir.1);
}

// Return if it caused infection
fn burst_part1(infected: &mut BTreeSet<(isize, isize)>, pos: &mut (isize, isize), dir: &mut (isize, isize)) -> bool {
    let mut caused_infection = false;
    if infected.contains(pos) {
        turn_right(dir);
        infected.remove(pos);
    } else {
        turn_left(dir);
        infected.insert(*pos);
        caused_infection = true;

    }
    pos.0 += dir.0;
    pos.1 += dir.1;

    caused_infection
}

fn part1(input: &str) -> usize {
    let mut infected = parse(input);
    let mut pos = (0, 0);
    let mut dir = (0, -1);

    let mut infected_bursts = 0;
    for _ in 0..10000 {
        if burst_part1(&mut infected, &mut pos, &mut dir) {
            infected_bursts += 1;
        }
    }

    infected_bursts
}

// Returns whether it caused infection
fn burst_part2(grid: &mut BTreeMap<(isize, isize), Status>, pos: &mut (isize, isize), dir: &mut (isize, isize)) -> bool {
    let mut caused_infection = false;
    match grid.get(pos).map(|s| *s) {
        None /* Clean */ => {
            grid.insert(*pos, Status::Weakened);
            turn_left(dir);
        },
        Some(Status::Weakened) => {
            grid.insert(*pos, Status::Infected);
            caused_infection = true;
        },
        Some(Status::Infected) => {
            grid.insert(*pos, Status::Flagged);
            turn_right(dir);
        },
        Some(Status::Flagged) => {
            grid.remove(pos);
            turn_around(dir);
        }
    }
    pos.0 += dir.0;
    pos.1 += dir.1;

    caused_infection
}

fn calc_part2(input: &str, iterations: usize) -> usize {
    let mut grid = parse(input).into_iter().map(|pos| (pos, Status::Infected)).collect();
    let mut pos = (0, 0);
    let mut dir = (0, -1);

    let mut infected_bursts = 0;
    for _ in 0..iterations {
        if burst_part2(&mut grid, &mut pos, &mut dir) {
            infected_bursts += 1;
        }
    }

    infected_bursts
}

fn part2(input: &str) -> usize {
    calc_part2(input, 10000000)
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
..#
#..
...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 5587);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc_part2(EX, 100), 26);
        assert_eq!(part2(EX), 2511944); // Warning: Takes a very long time in debug mode
    }

}
