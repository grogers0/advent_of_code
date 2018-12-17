use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

struct Instruction {
    write: bool,
    move_offset: isize,
    next_state: char
}
type StateInstructions = BTreeMap<bool, Instruction>;

fn parse(input: &str) -> (char, usize, BTreeMap<char, StateInstructions>) {
    lazy_static!{
        static ref BEGIN_RE: Regex = Regex::new("^Begin in state ([A-Z])\\.$").unwrap();
        static ref CHECKSUM_RE: Regex = Regex::new("^Perform a diagnostic checksum after (\\d+) steps\\.$").unwrap();
        static ref IN_STATE_RE: Regex = Regex::new("^In state ([A-Z]):$").unwrap();
        static ref WRITE_RE: Regex = Regex::new("^    - Write the value (\\d+)\\.$").unwrap();
        static ref MOVE_RE: Regex = Regex::new("^    - Move one slot to the (left|right)\\.$").unwrap();
        static ref NEXT_STATE_RE: Regex = Regex::new("^    - Continue with state ([A-Z])\\.$").unwrap();
    }

    let mut lines_iter = input.lines();
    let begin_state = BEGIN_RE.captures(lines_iter.next().unwrap()).unwrap()[1].chars().next().unwrap();
    let checksum_after = CHECKSUM_RE.captures(lines_iter.next().unwrap()).unwrap()[1].parse().unwrap();
    let mut instructions: BTreeMap<char, StateInstructions> = BTreeMap::new();
    while let Some("") = lines_iter.next() {
        let state = IN_STATE_RE.captures(lines_iter.next().unwrap()).unwrap()[1].chars().next().unwrap();
        let mut state_instructions = BTreeMap::new();
        for curr_val in [false, true].iter() {
            assert_eq!(&format!("  If the current value is {}:", if *curr_val { 1 } else { 0 }), lines_iter.next().unwrap());
            let write = &WRITE_RE.captures(lines_iter.next().unwrap()).unwrap()[1];
            let write = match write { "0" => false, "1" => true, _ => unreachable!() };
            let move_offset = &MOVE_RE.captures(lines_iter.next().unwrap()).unwrap()[1];
            let move_offset = match move_offset { "left" => -1, "right" => 1, _ => unreachable!() };
            let next_state = NEXT_STATE_RE.captures(lines_iter.next().unwrap()).unwrap()[1].chars().next().unwrap();
            state_instructions.insert(*curr_val, Instruction { write: write, move_offset: move_offset, next_state: next_state });
        }
        instructions.insert(state, state_instructions);
    }
    (begin_state, checksum_after, instructions)
}

fn diagnostic_checksum(tape: &BTreeSet<isize>) -> usize {
    tape.len()
}

fn write_val(tape: &mut BTreeSet<isize>, pos: isize, val: bool) {
    if val {
        tape.insert(pos);
    } else {
        tape.remove(&pos);
    }
}

fn part1(input: &str) -> usize {
    let (mut state, checksum_after, all_instructions) = parse(input);
    let mut tape = BTreeSet::new();
    let mut pos = 0;

    for _ in 0..checksum_after {
        let curr_val = tape.contains(&pos);
        let inst = all_instructions.get(&state).unwrap().get(&curr_val).unwrap();
        write_val(&mut tape, pos, inst.write);
        pos += inst.move_offset;
        state = inst.next_state;
    }

    diagnostic_checksum(&tape)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 3);
    }
}
