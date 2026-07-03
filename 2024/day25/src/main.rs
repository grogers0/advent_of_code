use std::io::{self, Read};

type PinHeights = [u8; 5];

fn parse_lock(schematic: &str) -> PinHeights {
    let mut pin_heights = [0, 0, 0, 0, 0];
    let mut lines = schematic.lines();
    assert_eq!(lines.next().unwrap(), "#####");
    for line in lines {
        assert_eq!(5, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => pin_heights[x] += 1,
                '.' => (),
                _ => panic!(),
            };
        }
    }
    pin_heights
}

fn parse_key(schematic: &str) -> PinHeights {
    let mut pin_heights = [5, 5, 5, 5, 5];
    let mut lines = schematic.lines();
    assert_eq!(lines.next().unwrap(), ".....");
    for line in lines {
        assert_eq!(5, line.chars().count());
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => (),
                '.' => pin_heights[x] -= 1,
                _ => panic!(),
            };
        }
    }
    pin_heights
}

fn parse(puzzle_input: &str) -> (Vec<PinHeights>, Vec<PinHeights>) {
    let mut locks = vec![];
    let mut keys = vec![];
    for schematic in puzzle_input.split("\n\n") {
        if schematic.lines().next().unwrap() == "#####" {
            locks.push(parse_lock(schematic));
        } else {
            keys.push(parse_key(schematic));
        }
    }
    (locks, keys)
}

fn is_overlapping(lock: &PinHeights, key: &PinHeights) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 { return true; }
    }
    false
}

fn part1(locks: &[PinHeights], keys: &[PinHeights]) -> usize {
    let mut cnt = 0;
    for lock in locks {
        for key in keys {
            if !is_overlapping(lock, key) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (locks, keys) = parse(&puzzle_input);
    println!("{}", part1(&locks, &keys));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part1() {
        let (locks, keys) = parse(EX);
        assert_eq!(part1(&locks, &keys), 3);
    }
}
