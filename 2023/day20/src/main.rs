use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

use num::integer::lcm;

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { inputs_memory: HashMap<String, bool> },
}

#[derive(Clone, Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
}

fn parse(puzzle_input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = puzzle_input.lines().map(|line| {
        let mut sp_iter = line.split(" -> ");
        let mut name = sp_iter.next().unwrap();
        let module_type = if name.starts_with("%") {
            name = &name[1..];
            ModuleType::FlipFlop { state: false }
        } else if name.starts_with("&") {
            name = &name[1..];
            ModuleType::Conjunction { inputs_memory: HashMap::new() }
        } else if name == "broadcaster" {
            ModuleType::Broadcaster
        } else {
            panic!();
        };
        let outputs = sp_iter.next().unwrap().split(", ")
            .map(|out| out.to_string()).collect();
        assert!(sp_iter.next().is_none());
        (name.to_string(), Module { module_type, outputs })
    }).collect();
    for (from, from_module) in modules.clone() {
        for to in from_module.outputs {
            if let Some(ref mut to_module) = modules.get_mut(&to) {
                if let ModuleType::Conjunction { inputs_memory } = &mut to_module.module_type {
                    inputs_memory.insert(from.to_string(), false);
                }
            }
        }
    }
    modules
}

fn part1(mut modules: HashMap<String, Module>) -> u64 {
    fn broadcast_outputs(from: &str, outputs: &[String], is_high: bool, pulses: &mut VecDeque<(String, String, bool)>) {
        for output in outputs {
            pulses.push_back((from.to_string(), output.to_string(), is_high));
        }
    }
    let mut cnt_high = 0;
    let mut cnt_low = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while let Some((from, to, is_high)) = pulses.pop_front() {
            if is_high { cnt_high += 1 } else { cnt_low += 1 };
            if let Some(ref mut module) = modules.get_mut(&to) {
                match &mut module.module_type {
                    ModuleType::Broadcaster =>
                        broadcast_outputs(&to, &module.outputs, is_high, &mut pulses),
                        ModuleType::FlipFlop { state } => {
                            if !is_high {
                                *state = !*state;
                                broadcast_outputs(&to, &module.outputs, *state, &mut pulses);
                            }
                        },
                        ModuleType::Conjunction { ref mut inputs_memory } => {
                            inputs_memory.insert(from.clone(), is_high);
                            broadcast_outputs(&to, &module.outputs, !inputs_memory.values().all(|v| *v), &mut pulses);
                        }
                };
            }
        }
    }
    cnt_high * cnt_low
}

fn get_inputs_to(modules: &HashMap<String, Module>, to: &str) -> HashSet<String> {
    let mut ret = HashSet::new();
    for (name, module) in modules {
        if module.outputs.iter().any(|e| e == to) {
            ret.insert(name.to_string());
        }
    }
    ret
}

fn part2(mut modules: HashMap<String, Module>) -> u64 {
    fn broadcast_outputs(from: &str, outputs: &[String], is_high: bool, pulses: &mut VecDeque<(String, String, bool)>) {
        for output in outputs {
            pulses.push_back((from.to_string(), output.to_string(), is_high));
        }
    }

    // There's a weird structure to the input where several cycles of different length feed into a
    // single conjunction which feeds into the output...
    let conjunction_name = get_inputs_to(&modules, "rx");
    assert_eq!(conjunction_name.len(), 1);
    let conjunction_name = conjunction_name.into_iter().next().unwrap();
    assert!(matches!(modules[&conjunction_name].module_type, ModuleType::Conjunction { .. }));
    let conjunction_inputs = get_inputs_to(&modules, &conjunction_name);
    let mut cycles = HashMap::new();

    for i in 1.. {
        if cycles.len() == conjunction_inputs.len() { break }
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while let Some((from, to, is_high)) = pulses.pop_front() {
            if conjunction_name == to && is_high { cycles.entry(from.to_string()).or_insert(i); }
            if let Some(ref mut module) = modules.get_mut(&to) {
                match &mut module.module_type {
                    ModuleType::Broadcaster =>
                        broadcast_outputs(&to, &module.outputs, is_high, &mut pulses),
                        ModuleType::FlipFlop { state } => {
                            if !is_high {
                                *state = !*state;
                                broadcast_outputs(&to, &module.outputs, *state, &mut pulses);
                            }
                        },
                        ModuleType::Conjunction { ref mut inputs_memory } => {
                            inputs_memory.insert(from.clone(), is_high);
                            broadcast_outputs(&to, &module.outputs, !inputs_memory.values().all(|v| *v), &mut pulses);
                        }
                };
            }
        }
    }
    cycles.values().cloned().reduce(lcm).unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let modules = parse(&puzzle_input);
    println!("{}", part1(modules.clone()));
    println!("{}", part2(modules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EX2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(EX1)), 32000000);
        assert_eq!(part1(parse(EX2)), 11687500);
    }
}
