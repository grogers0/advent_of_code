use std::io::{self, Read};
use std::collections::{BTreeMap, BinaryHeap, HashMap};

struct Pos {
    x: u64,
    y: u64,
    z: u64,
}

fn parse(puzzle_input: &str) -> Vec<Pos> {
    let mut ret = vec![];
    for line in puzzle_input.lines() {
        let mut sp = line.split(",");
        let x = sp.next().unwrap().parse::<u64>().unwrap();
        let y = sp.next().unwrap().parse::<u64>().unwrap();
        let z = sp.next().unwrap().parse::<u64>().unwrap();
        assert!(sp.next().is_none());
        ret.push(Pos { x, y, z });
    }
    ret
}

fn squared_dist(a: &Pos, b: &Pos) -> u64 {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    let dz = if a.z > b.z { a.z - b.z } else { b.z - a.z };
    dx*dx + dy*dy + dz*dz
}

// NOTE - assumes that all distances are unique, this seems to be the case
fn calc_paired_distances(positions: &[Pos]) -> BTreeMap<u64, [usize; 2]> {
    let mut ret = BTreeMap::new();
    for i in 0..(positions.len() - 1) {
        for j in (i + 1)..positions.len() {
            let prior = ret.insert(
                squared_dist(&positions[i], &positions[j]), [i, j]);
            if prior.is_some() {
                panic!("current impl doesn't support duplicate distances");
            }
        }
    }
    ret
}

fn part1(positions: &[Pos], connections_to_make: usize) -> usize {
    let mut paired_distances = calc_paired_distances(positions);

    let mut pos_to_circuit = HashMap::new();
    let mut circuit_to_pos = HashMap::new();
    for i in 0..positions.len() {
        pos_to_circuit.insert(i, i);
        circuit_to_pos.insert(i, vec![i]);
    }

    for _ in 0..connections_to_make {
        let (dist, [a, b]) = paired_distances.first_key_value().unwrap();
        let a = *a;
        let b = *b;
        let dist = *dist;
        paired_distances.remove(&dist);
        let cir_a = pos_to_circuit[&a];
        let cir_b = pos_to_circuit[&b];
        if cir_a == cir_b { continue; }
        let pos_ids = circuit_to_pos.remove(&cir_b).unwrap();
        for p in pos_ids {
            pos_to_circuit.insert(p, cir_a);
            circuit_to_pos.get_mut(&cir_a).unwrap().push(p);
        }
    }

    let mut circuit_sizes = BinaryHeap::new();
    for (_, pos_ids) in circuit_to_pos.iter() {
        circuit_sizes.push(pos_ids.len());
    }
    
    let mut product = 1;
    for _ in 0..3 {
        product *= circuit_sizes.pop().unwrap();
    }
    product
}

fn part2(positions: &[Pos]) -> u64 {
    let mut paired_distances = calc_paired_distances(positions);

    let mut pos_to_circuit = HashMap::new();
    let mut circuit_to_pos = HashMap::new();
    for i in 0..positions.len() {
        pos_to_circuit.insert(i, i);
        circuit_to_pos.insert(i, vec![i]);
    }

    let mut lasta = 0;
    let mut lastb = 0;
    while circuit_to_pos.len() > 1 {
        let (dist, [a, b]) = paired_distances.first_key_value().unwrap();
        let a = *a;
        let b = *b;
        let dist = *dist;
        paired_distances.remove(&dist);
        let cir_a = pos_to_circuit[&a];
        let cir_b = pos_to_circuit[&b];
        if cir_a == cir_b { continue; }
        lasta = a;
        lastb = b;
        let pos_ids = circuit_to_pos.remove(&cir_b).unwrap();
        for p in pos_ids {
            pos_to_circuit.insert(p, cir_a);
            circuit_to_pos.get_mut(&cir_a).unwrap().push(p);
        }
    }

    positions[lasta].x * positions[lastb].x
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let positions = parse(&puzzle_input);
    println!("{}", part1(&positions, 1000));
    println!("{}", part2(&positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX), 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 25272);
    }
}
