use std::io::{self, Read};
use std::cmp::Reverse;
use std::collections::{HashSet, BinaryHeap};

type Mask = u16;

struct Machine {
    target: Mask,
    buttons: Vec<Mask>,
    jolts: Vec<usize>,
}

fn parse_light_target(input: &str) -> Mask {
    assert_eq!("[", &input[0..=0]);
    assert_eq!("]", &input[(input.len() - 1)..=(input.len() - 1)]);
    let mut curr = 1;
    let mut ret = 0;
    for ch in input[1..(input.len()-1)].chars() {
        match ch {
            '#' => ret |= curr,
            '.' => (),
            _ => panic!(),
        }
        curr <<= 1;
    }
    ret
}

fn parse_button(input: &str) -> Mask {
    assert_eq!("(", &input[0..=0]);
    assert_eq!(")", &input[(input.len() - 1)..=(input.len() - 1)]);

    let mut ret = 0;
    for s in input[1..(input.len() - 1)].split(",") {
        ret |= 1 << s.parse::<Mask>().unwrap();
    }
    ret
}

fn parse_joltages(input: &str) -> Vec<usize> {
    assert_eq!("{", &input[0..=0]);
    assert_eq!("}", &input[(input.len() - 1)..=(input.len() - 1)]);

    let mut ret = vec![];
    for s in input[1..(input.len() - 1)].split(",") {
        ret.push(s.parse::<usize>().unwrap());
    }
    ret
}

fn parse(puzzle_input: &str) -> Vec<Machine> {
    let mut ret = vec![];
    for line in puzzle_input.lines() {
        let mut sp = line.split(" ");
        let target = parse_light_target(sp.next().unwrap());
        let mut inputs = sp.collect::<Vec<&str>>();
        let jolts = parse_joltages(inputs.pop().unwrap());
        let buttons =
            inputs.into_iter().map(|input| parse_button(input)).collect();
        ret.push(Machine { target, buttons, jolts });
    }
    ret
}

// Returns a Vec of Mask (representing the buttons pressed), sorted by total
// buttons pressed
fn compute_for_target(buttons: &[Mask], target: Mask) -> Vec<Mask> {
    fn recur(buttons: &[Mask], target: Mask, i: usize, path: Mask, results: &mut Vec<Mask>) {
        if i == buttons.len() {
            if target == 0 { results.push(path); }
            return;
        }

        recur(buttons, target, i + 1, path, results);
        recur(buttons, target ^ buttons[i], i + 1, path | (1 << i), results);
    }

    let mut results = Vec::new();
    recur(buttons, target, 0, 0, &mut results);
    results.sort_unstable_by_key(|pressed| pressed.count_ones());
    results
}

fn part1(machines: &[Machine]) -> usize {
    let mut ret = 0;
    for machine in machines {
        ret += compute_for_target(&machine.buttons, machine.target)
            .first().unwrap().count_ones() as usize;
    }
    ret
}

fn get_parity(jolts: &[usize]) -> Mask {
    let mut mask = 0;
    for (i, v) in jolts.iter().enumerate() {
        if v % 2 == 1 {
            mask |= 1 << i;
        }
    }
    mask
}

fn do_presses(buttons: &[Mask], presses: Mask, mut jolts: Vec<usize>)
    -> Option<Vec<usize>> {
    for i in 0..buttons.len() {
        if presses & (1 << i) != 0 {
            for j in 0..jolts.len() {
                if buttons[i] & (1 << j) == 0 { continue; }
                if jolts[j] == 0 { return None; }
                jolts[j] -= 1;
            }
        }
    }
    Some(jolts)
}

fn count_presses(buttons: &[Mask], jolts: Vec<usize>) -> Option<usize> {
    if jolts.iter().all(|&v| v == 0) { return Some(0); }

    let mut min_presses: Option<usize> = None;
    for init_presses in compute_for_target(buttons, get_parity(&jolts)) {
        if let Some(mut jolts) = do_presses(buttons, init_presses, jolts.clone()) {
            for v in &mut jolts { *v /= 2; }
            if let Some(subs_num_presses) = count_presses(buttons, jolts) {
                let num_presses = init_presses.count_ones() as usize
                    + 2 * subs_num_presses;
                if min_presses.is_none()
                    || num_presses < min_presses.unwrap() {
                        min_presses = Some(num_presses);
                }
            }
        }
    }
    min_presses
}

// Using https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
fn part2(machines: &[Machine]) -> usize {
    let mut ret = 0;
    for machine in machines {
        ret += count_presses(&machine.buttons, machine.jolts.clone()).unwrap();
    }
    ret
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let machines = parse(&puzzle_input);
    println!("{}", part1(&machines));
    println!("{}", part2(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 33);
    }
}
