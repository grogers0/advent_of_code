use std::cmp::Ordering;
use std::io::{self, Read};

use lazy_static::lazy_static;
use num_integer::lcm;
use regex::Regex;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3]
}

fn parse(input: &str) -> Vec<Moon> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^<x=([0-9-]+), y=([0-9-]+), z=([0-9-]+)>$").unwrap();
    }

    input.trim().lines().map(|line| {
        let cap = RE.captures(line).unwrap();
        let pos = [
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap()
        ];
        Moon {
            pos: pos,
            vel: [0, 0, 0]
        }
    })
    .collect()
}

fn get_mut_pair<T>(v: &mut Vec<T>, i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i < j);
    let (a, b) = v.split_at_mut(j);
    (&mut a[i], &mut b[0])
}

fn step_velocities(moons: &mut Vec<Moon>) {
    for i in 0 .. moons.len() {
        for j in i+1 .. moons.len() {
            let (m1, m2) = get_mut_pair(moons, i, j);
            for axis in 0 .. 3 {
                match m1.pos[axis].cmp(&m2.pos[axis]) {
                    Ordering::Less => {
                        m1.vel[axis] += 1;
                        m2.vel[axis] -= 1;
                    },
                    Ordering::Greater => {
                        m1.vel[axis] -= 1;
                        m2.vel[axis] += 1;
                    },
                    Ordering::Equal => ()
                }
            }
        }
    }
}

fn step_positions(moons: &mut Vec<Moon>) {
    for moon in moons.iter_mut() {
        for axis in 0 .. 3 {
            moon.pos[axis] += moon.vel[axis];
        }
    }
}

fn step(moons: &mut Vec<Moon>) {
    step_velocities(moons);
    step_positions(moons);
}

fn potential_energy(moon: &Moon) -> u64 {
    (0 .. 3).map(|axis| moon.pos[axis].abs() as u64).sum()
}

fn kinetic_energy(moon: &Moon) -> u64 {
    (0 .. 3).map(|axis| moon.vel[axis].abs() as u64).sum()
}

fn energy(moon: &Moon) -> u64 {
    potential_energy(moon) * kinetic_energy(moon)
}

fn total_energy(moons: &Vec<Moon>) -> u64 {
    moons.iter().map(|moon| energy(moon)).sum()
}

fn axis_repeats(moons1: &Vec<Moon>, moons2: &Vec<Moon>, axis: usize) -> bool {
    moons1.iter().zip(moons2.iter()).all(|(moon1, moon2)| {
        moon1.pos[axis] == moon2.pos[axis] && moon1.vel[axis] == moon2.vel[axis]
    })
}

fn steps_before_repeat(moons: &mut Vec<Moon>) -> u64 {
    let initial_moons = moons.clone();
    let mut steps_for_axis_repeat = [0; 3];
    for steps in 1 .. {
        if steps_for_axis_repeat.iter().all(|steps_for_repeat| *steps_for_repeat != 0) {
            break;
        }
        step(moons);
        for axis in 0 .. 3 {
            if steps_for_axis_repeat[axis] == 0 && axis_repeats(&initial_moons, moons, axis) {
                steps_for_axis_repeat[axis] = steps;
            }
        }
    }
    lcm(steps_for_axis_repeat[0], lcm(steps_for_axis_repeat[1], steps_for_axis_repeat[2]))
}

fn part1(input: &str) -> u64 {
    let mut moons = parse(input);
    for _ in 0 .. 1000 {
        step(&mut moons);
    }
    total_energy(&moons)
}

fn part2(input: &str) -> u64 {
    steps_before_repeat(&mut parse(input))
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

    const EX1: &str = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const EX2: &str = "
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn test_part1() {
        let mut moons = parse(EX1);
        for _ in 0 .. 10 {
            step(&mut moons);
        }
        let ex_end = vec![
            Moon { pos: [2,  1, -3], vel: [-3, -2,  1] },
            Moon { pos: [1, -8,  0], vel: [-1,  1,  3] },
            Moon { pos: [3, -6,  1], vel: [ 3,  2, -3] },
            Moon { pos: [2,  0,  4], vel: [ 1, -1, -1] }
        ];
        assert_eq!(ex_end, moons);
        assert_eq!(179, total_energy(&moons));

        let mut moons = parse(EX2);
        for _ in 0 .. 100 {
            step(&mut moons);
        }
        let ex_end = vec![
            Moon { pos: [  8, -12, -9], vel: [-7,   3,  0] },
            Moon { pos: [ 13,  16, -3], vel: [ 3, -11, -5] },
            Moon { pos: [-29, -11, -1], vel: [-3,   7,  4] },
            Moon { pos: [ 16, -13, 23], vel: [ 7,   1,  1] }
        ];
        assert_eq!(ex_end, moons);
        assert_eq!(1940, total_energy(&moons));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2772, steps_before_repeat(&mut parse(EX1)));
        assert_eq!(4686774924, steps_before_repeat(&mut parse(EX2)));
    }
}
