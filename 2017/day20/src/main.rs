use std::collections::BTreeMap;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

struct Particle {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64)
}

fn parse(input: &str) -> BTreeMap<usize, Particle> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^p=<([0-9-]+),([0-9-]+),([0-9-]+)>, v=<([0-9-]+),([0-9-]+),([0-9-]+)>, a=<([0-9-]+),([0-9-]+),([0-9-]+)>$").unwrap();
    }

    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Particle {
                position: (cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                velocity: (cap[4].parse().unwrap(), cap[5].parse().unwrap(), cap[6].parse().unwrap()),
                acceleration: (cap[7].parse().unwrap(), cap[8].parse().unwrap(), cap[9].parse().unwrap())
            }
        })
        .enumerate()
        .collect()
}

fn step_particle(p: &mut Particle) {
    p.velocity.0 += p.acceleration.0;
    p.velocity.1 += p.acceleration.1;
    p.velocity.2 += p.acceleration.2;

    p.position.0 += p.velocity.0;
    p.position.1 += p.velocity.1;
    p.position.2 += p.velocity.2;
}

fn part1(input: &str) -> usize {
    let mut particles = parse(input);
    for _ in 0..1000 { // arbitrary upper bound
        for p in particles.values_mut() {
            step_particle(p);
        }
    }

    *particles.iter()
        .min_by_key(|(_,p)| p.position.0.abs() + p.position.1.abs() + p.position.2.abs())
        .unwrap().0
}

fn part2(input: &str) -> usize {
    let mut particles = parse(input);
    for _ in 0..1000 { // arbitrary upper bound
        for p in particles.values_mut() {
            step_particle(p);
        }
        let mut seen = BTreeMap::new();
        for p in particles.values() {
            seen.entry(p.position).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        for id in particles.keys().cloned().collect::<Vec<usize>>() {
            if seen[&particles[&id].position] > 1 {
                particles.remove(&id);
            }
        }
    }

    particles.len()
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
        let ex = "\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        assert_eq!(part1(ex), 0);
    }


    #[test]
    fn test_part2() {
        let ex = "\
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(part2(ex), 1);
    }

}
