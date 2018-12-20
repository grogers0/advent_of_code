use std::collections::{BTreeSet, VecDeque};
use std::io::{self, Read};
use std::mem::discriminant;

use arrayvec::ArrayVec;
use lazy_static::lazy_static;
use regex::Regex;
use string_interner::{StringInterner, Symbol};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Part {
    Microchip(Sym),
    Generator(Sym)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Sym {
    val: u8
}

impl Symbol for Sym {
    fn from_usize(val: usize) -> Sym {
        if val > std::u8::MAX as usize { panic!() }
        Sym { val: val as u8 }
    }
    fn to_usize(self) -> usize {
        self.val as usize
    }
}

const FLOORS: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Map {
    elevator: usize,
    floors: [BTreeSet<Part>; FLOORS]
}

fn parse_parts(input: &str, interner: &mut StringInterner<Sym>) -> [BTreeSet<Part>; FLOORS] {
    lazy_static!{
        static ref MICROCHIP_RE: Regex = Regex::new("([a-z]+)-compatible microchip").unwrap();
        static ref GENERATOR_RE: Regex = Regex::new("([a-z]+) generator").unwrap();
    }
    let mut floors = ArrayVec::<[_; FLOORS]>::new();
    for line in input.lines() {
        let mut floor: BTreeSet<_> = MICROCHIP_RE.captures_iter(line).map(|cap| Part::Microchip(interner.get_or_intern(&cap[1]))).collect();
        floor.append(&mut GENERATOR_RE.captures_iter(line).map(|cap| Part::Generator(interner.get_or_intern(&cap[1]))).collect());
        floors.push(floor);
    }
    floors.into_inner().unwrap()
}

fn is_complete(map: &Map) -> bool {
    map.floors[0..FLOORS-1].iter().all(|floor| floor.is_empty())
}

fn is_safe(map: &Map) -> bool {
    let safe_floor = |floor: &BTreeSet<Part>| {
        for part in floor.iter() {
            if let Part::Microchip(m) = part {
                if !floor.contains(&Part::Generator(*m)) &&
                    floor.iter().any(|p2| discriminant(p2) == discriminant(&Part::Generator(Sym { val: 0 })))
                {
                    return false;
                }
            }
        }
        true
    };
    map.floors.iter().all(|floor| safe_floor(floor))
}

fn possible_floors(floor: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    if floor > 0 { ret.push(floor - 1) }
    if floor < FLOORS - 1 { ret.push(floor + 1) }
    ret
}

fn possible_parts(parts: &BTreeSet<Part>) -> Vec<Vec<&Part>> {
    let parts = parts.iter().collect::<Vec<_>>();
    let mut ret = Vec::new();
    for part in parts.iter() {
        ret.push(vec![*part]);
    }
    for i in 0..parts.len() {
        for j in i+1 .. parts.len() {
            ret.push(vec![parts[i], parts[j]]);
        }
    }
    ret
}

fn move_elevator(map: &Map, next_floor: usize, parts_to_bring: &Vec<&Part>) -> Map {
    let mut map = map.clone();
    for part in parts_to_bring.iter() {
        map.floors[map.elevator].remove(part);
        map.floors[next_floor].insert((*part).clone());
    }
    map.elevator = next_floor;
    map
}

fn num_steps_to_complete(map: Map) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, map));
    let mut seen = BTreeSet::new();

    while let Some((steps, map)) = queue.pop_front() {
        if is_complete(&map) { return steps; }
        if !is_safe(&map) { continue; }
        if !seen.insert(map.clone()) { continue; }

        let next_floors = possible_floors(map.elevator);
        for parts_to_bring in possible_parts(&map.floors[map.elevator]) {
            for next_floor in next_floors.iter() {
                queue.push_back((steps + 1, move_elevator(&map, *next_floor, &parts_to_bring)));
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    let mut interner = StringInterner::new();
    let map = Map { elevator: 0, floors: parse_parts(input, &mut interner) };
    num_steps_to_complete(map)
}

// TODO - this takes several minutes and several GB of RAM to complete, even with the string
// interning optimization. I suspect instead of exhaustive BFS, something like A* would be better.
// Maybe there is a hidden trick to solve this without enumerating the actual path taken?
fn part2(input: &str) -> usize {
    let mut interner = StringInterner::new();
    let mut map = Map { elevator: 0, floors: parse_parts(input, &mut interner) };
    map.floors[0].insert(Part::Microchip(interner.get_or_intern("elerium")));
    map.floors[0].insert(Part::Generator(interner.get_or_intern("elerium")));
    map.floors[0].insert(Part::Microchip(interner.get_or_intern("dilithium")));
    map.floors[0].insert(Part::Generator(interner.get_or_intern("dilithium")));
    num_steps_to_complete(map)
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
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 11);
    }
}
