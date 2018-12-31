use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Read};

use regex::Regex;

#[derive(Clone, Debug)]
enum Elem {
    Set(u16, String),
    Cpy(String, String),
    And(String, String, String),
    AndVal(u16, String, String),
    Or(String, String, String),
    Lshift(String, u16, String),
    Rshift(String, u16, String),
    Not(String, String)
}

fn parse_circuit(input: &str) -> Vec<Elem> {
    let set_re = Regex::new("^(\\d+|[a-z]+) -> ([a-z]+)$").unwrap();
    let shift_re = Regex::new("^([a-z]+) (LSHIFT|RSHIFT) (\\d+) -> ([a-z]+)$").unwrap();
    let binary_re = Regex::new("^(\\d+|[a-z]+) (AND|OR) ([a-z]+) -> ([a-z]+)$").unwrap();
    let not_re = Regex::new("^NOT ([a-z]+) -> ([a-z]+)$").unwrap();
    input.lines()
        .map(|line| {
            if let Some(cap) = set_re.captures(line) {
                if let Ok(n) = cap[1].parse() {
                    Elem::Set(n, cap[2].to_string())
                } else {
                    Elem::Cpy(cap[1].to_string(), cap[2].to_string())
                }
            } else if let Some(cap) = shift_re.captures(line) {
                let x = cap[1].to_string();
                let y = cap[3].parse().unwrap();
                let z = cap[4].to_string();
                match &cap[2] {
                    "LSHIFT" => Elem::Lshift(x, y, z),
                    "RSHIFT" => Elem::Rshift(x, y, z),
                    _ => panic!()
                }
            } else if let Some(cap) = binary_re.captures(line) {
                let x = cap[1].to_string();
                let y = cap[3].to_string();
                let z = cap[4].to_string();
                match &cap[2] {
                    "AND" => {
                        if let Ok(n) = cap[1].parse() {
                            Elem::AndVal(n, y, z)
                        } else {
                            Elem::And(x, y, z)
                        }
                    },
                    "OR" => Elem::Or(x, y, z),
                    _ => panic!()
                }
            } else if let Some(cap) = not_re.captures(line) {
                Elem::Not(cap[1].to_string(), cap[2].to_string())
            } else {
                panic!(line.to_string())
            }
        })
        .collect()
}

fn ready(signals: &BTreeMap<String, u16>, wires: &[&str]) -> bool {
    wires.iter().all(|wire| signals.contains_key(*wire))
}

fn set(signals: &mut BTreeMap<String, u16>, wire: &str, val: u16) {
    // Ignore is the value is already set
    signals.entry(wire.to_string()).or_insert(val);
}

fn simulate(circuit: &Vec<Elem>, signals: &mut BTreeMap<String, u16>) {
    let mut circuit = VecDeque::from(circuit.clone());
    while let Some(elem) = circuit.pop_front() {
        match &elem {
            Elem::Set(v, z) => set(signals, z, *v),
            Elem::Cpy(x, z) => {
                if !ready(&signals, &[x]) { circuit.push_back(elem); }
                else { set(signals, z, signals[x]); }
            },
            Elem::And(x, y, z) => {
                if !ready(&signals, &[x, y]) { circuit.push_back(elem); }
                else { set(signals, z, signals[x] & signals[y]); }
            },
            Elem::AndVal(n, y, z) => {
                if !ready(&signals, &[y]) { circuit.push_back(elem); }
                else { set(signals, z,  n & signals[y]); }
            },
            Elem::Or(x, y, z) => {
                if !ready(&signals, &[x, y]) { circuit.push_back(elem); }
                else { set(signals, z, signals[x] | signals[y]); }
            },
            Elem::Lshift(x, n, z) => {
                if !ready(&signals, &[x]) { circuit.push_back(elem); }
                else { set(signals, z, signals[x] << n); }
            },
            Elem::Rshift(x, n, z) => {
                if !ready(&signals, &[x]) { circuit.push_back(elem); }
                else { set(signals, z, signals[x] >> n); }
            },
            Elem::Not(x, z) => {
                if !ready(&signals, &[x]) { circuit.push_back(elem); }
                else { set(signals, z, !signals[x]); }
            }
        }
    }
}

fn part1(input: &str) -> u16 {
    let mut signals = BTreeMap::new();
    simulate(&parse_circuit(input), &mut signals);
    signals["a"]
}

fn part2(input: &str) -> u16 {
    let circuit = parse_circuit(input);
    let mut signals = BTreeMap::new();
    simulate(&circuit, &mut signals);
    let mut signals2 = BTreeMap::new();
    signals2.insert("b".to_string(), signals["a"]);
    simulate(&circuit, &mut signals2);
    signals2["a"]
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
        let circuit_str = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let mut expected_signals = BTreeMap::new();
        set(&mut expected_signals, "d", 72);
        set(&mut expected_signals, "e", 507);
        set(&mut expected_signals, "f", 492);
        set(&mut expected_signals, "g", 114);
        set(&mut expected_signals, "h", 65412);
        set(&mut expected_signals, "i", 65079);
        set(&mut expected_signals, "x", 123);
        set(&mut expected_signals, "y", 456);

        let mut signals = BTreeMap::new();
        simulate(&parse_circuit(circuit_str), &mut signals);
        assert_eq!(signals, expected_signals);
    }
}
