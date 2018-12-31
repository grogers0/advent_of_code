use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

use day12_2016::*;

fn toggle_clock(clock: i64) -> i64 {
    if clock == 0 { 1 } else { 0 }
}

fn check_toggling_clock(mut program: Vec<Op>, mut registers: BTreeMap<String, i64>) -> bool {
    let mut seen = BTreeSet::new();
    let mut pc = 0i64;
    let mut last_clock = 1;
    loop {
        if pc < 0 && pc >= program.len() as i64 { return false }
        if !seen.insert((program.clone(), registers.clone(), pc)) {
            break;
        }
        let mut clock = None;
        execute_op(&mut program, &mut pc, &mut clock, &mut registers);
        if let Some(clock) = clock {
            if toggle_clock(last_clock) != clock {
                return false
            }
            last_clock = clock;
        }
    }

    let saved_state = (program.clone(), registers.clone(), pc);
    let mut clock_changes = 0;
    let mut cycle_len = 0;

    while pc >= 0 && pc < program.len() as i64 {
        if cycle_len != 0 && saved_state.0 == program && saved_state.1 == registers && saved_state.2 == pc {
            return clock_changes > 0;
        }
        cycle_len += 1;

        let mut clock = None;
        execute_op(&mut program, &mut pc, &mut clock, &mut registers);
        if let Some(clock) = clock {
            if toggle_clock(last_clock) != clock {
                return false
            }
            last_clock = clock;
            clock_changes += 1;
        }
    }
    false
}


fn part1(input: &str) -> i64 {
    let program = parse_ops(input);
    for i in 0.. {
        let mut registers = BTreeMap::new();
        registers.insert("a".to_string(), i);
        if check_toggling_clock(program.clone(), registers) {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
}
