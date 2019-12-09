use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::sync::mpsc::{Receiver, Sender};

#[derive(PartialEq, Debug)]
pub struct Mem(HashMap<usize, i64>);

impl Index<usize> for Mem {
    type Output = i64;
    fn index(&self, idx: usize) -> &i64 {
        self.0.get(&idx).or(Some(&0)).unwrap()
    }
}

impl IndexMut<usize> for Mem {
    fn index_mut(&mut self, idx: usize) -> &mut i64 {
        self.0.entry(idx).or_insert(0)
    }
}

pub fn parse(mem_str: &str) -> Mem {
    Mem(mem_str.trim().split(",")
        .map(|chunk| chunk.parse().unwrap())
        .enumerate()
        .collect())
}

enum ParamMode {
    Position,
    Immediate,
    Relative
}

fn param_mode(mem: &Mem, pc: usize, param_num: usize) -> ParamMode {
    match mem[pc] / i64::pow(10, param_num as u32 + 1) % 10 {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        mode => unimplemented!("unknown parameter mode {}", mode)
    }
}

fn param(mem: &Mem, pc: usize, param_num: usize, relative_base: &i64) -> i64 {
    match param_mode(mem, pc, param_num) {
        ParamMode::Position => mem[mem[pc + param_num] as usize],
        ParamMode::Immediate => mem[pc + param_num],
        ParamMode::Relative => mem[(mem[pc + param_num] + *relative_base) as usize]
    }
}

fn param_mut<'a>(mem: &'a mut Mem, pc: usize, param_num: usize, relative_base: &i64) -> &'a mut i64 {
    match param_mode(mem, pc, param_num) {
        ParamMode::Position => {
            let offset = mem[pc + param_num];
            &mut mem[offset as usize]
        },
        ParamMode::Immediate => panic!("parameter {} of instruction {} at pc {} is a write in immediate mode",
            param_num, inst(mem, pc), pc),
        ParamMode::Relative => {
            let offset = mem[pc + param_num] + *relative_base;
            &mut mem[offset as usize]
        }
    }
}

fn inst(mem: &Mem, pc: usize) -> i64 {
    mem[pc] % 100
}

pub fn run(mem: &mut Mem, input: Receiver<i64>, output: Sender<i64>) {
    let mut pc = 0;
    let mut relative_base = 0;
    loop {
        match inst(mem, pc) {
            1 => { // add
                let a = param(mem, pc, 1, &relative_base);
                let b = param(mem, pc, 2, &relative_base);
                *param_mut(mem, pc, 3, &relative_base) = a + b;
                pc += 4;
            },
            2 => { // mul
                let a = param(mem, pc, 1, &relative_base);
                let b = param(mem, pc, 2, &relative_base);
                *param_mut(mem, pc, 3, &relative_base) = a * b;
                pc += 4;
            },
            3 => { // read input
                *param_mut(mem, pc, 1, &relative_base) = input.recv().unwrap();
                pc += 2;
            },
            4 => { // write output
                output.send(param(mem, pc, 1, &relative_base)).unwrap();
                pc += 2;
            },
            5 => { // jump if nonzero
                if param(mem, pc, 1, &relative_base) != 0 {
                    pc = param(mem, pc, 2, &relative_base) as usize;
                } else {
                    pc += 3;
                }
            },
            6 => { // jump if zero
                if param(mem, pc, 1, &relative_base) == 0 {
                    pc = param(mem, pc, 2, &relative_base) as usize;
                } else {
                    pc += 3;
                }
            },
            7 => { // less than
                let a = param(mem, pc, 1, &relative_base);
                let b = param(mem, pc, 2, &relative_base);
                *param_mut(mem, pc, 3, &relative_base) = if a < b { 1 } else { 0 };
                pc += 4;
            },
            8 => { // equals
                let a = param(mem, pc, 1, &relative_base);
                let b = param(mem, pc, 2, &relative_base);
                *param_mut(mem, pc, 3, &relative_base) = if a == b { 1 } else { 0 };
                pc += 4;
            },
            9 => { // Relative base offset
                relative_base += param(mem, pc, 1, &relative_base);
                pc += 2;
            }
            99 => break, // halt
            opcode => unimplemented!("unknown opcode {}", opcode)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use std::sync::mpsc::channel;

    use super::*;

    fn run_no_io(mem_str: &str) -> Mem {
        let mut mem = parse(mem_str);
        run(&mut mem, channel().1, channel().0);
        mem
    }

    fn run_single_io(mem_str: &str, input: i64) -> i64 {
        let (tx_in, rx_in) = channel();
        let (tx_out, rx_out) = channel();
        tx_in.send(input).unwrap();
        run(&mut parse(mem_str), rx_in, tx_out);
        let output = rx_out.recv().unwrap();
        assert!(rx_out.recv().is_err());
        output
    }

    #[test]
    fn test_day2_part1() {
        assert_eq!(run_no_io("1,9,10,3,2,3,11,0,99,30,40,50")[0], 3500);
        assert_eq!(run_no_io("1,0,0,0,99")[0], 2);
        assert_eq!(run_no_io("2,3,0,3,99")[3], 6);
        assert_eq!(run_no_io("2,3,0,3,99")[3], 6);
        assert_eq!(run_no_io("2,4,4,5,99,0")[5], 9801);
        assert_eq!(run_no_io("1,1,1,4,99,5,6,0,99")[0], 30);
    }

    #[test]
    fn test_day5_part1() {
        assert_eq!(run_no_io("1002,4,3,4,33"), parse("1002,4,3,4,99"));
    }

    #[test]
    fn test_day5_part2() {
        // Position mode
        let ex_eq = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(0, run_single_io(ex_eq, 7));
        assert_eq!(1, run_single_io(ex_eq, 8));
        assert_eq!(0, run_single_io(ex_eq, 9));

        let ex_lt = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(1, run_single_io(ex_lt, 7));
        assert_eq!(0, run_single_io(ex_lt, 8));
        assert_eq!(0, run_single_io(ex_lt, 9));

        let ex_nz = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(0, run_single_io(ex_nz, 0));
        assert_eq!(1, run_single_io(ex_nz, 1));
        assert_eq!(1, run_single_io(ex_nz, 2));

        // Immediate mode
        let ex_eq = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(0, run_single_io(ex_eq, 7));
        assert_eq!(1, run_single_io(ex_eq, 8));
        assert_eq!(0, run_single_io(ex_eq, 9));

        let ex_lt = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(1, run_single_io(ex_lt, 7));
        assert_eq!(0, run_single_io(ex_lt, 8));
        assert_eq!(0, run_single_io(ex_lt, 9));

        let ex_nz = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(0, run_single_io(ex_nz, 0));
        assert_eq!(1, run_single_io(ex_nz, 1));
        assert_eq!(1, run_single_io(ex_nz, 2));

        // Larger example
        let ex = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, run_single_io(ex, 7));
        assert_eq!(1000, run_single_io(ex, 8));
        assert_eq!(1001, run_single_io(ex, 9));
    }

    #[test]
    fn test_day9_part1() {
        let ex_quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let (tx_out, rx_out) = channel();
        run(&mut parse(ex_quine), channel().1, tx_out);
        let mut quine_out = String::new();
        let mut first = true;
        while let Ok(v) = rx_out.recv() {
            if first { first = false; } else { quine_out.push(',') }
            write!(quine_out, "{}", v).unwrap();
        }
        assert_eq!(quine_out, ex_quine);

        let ex_16digit = "1102,34915192,34915192,7,4,7,99,0";
        let (tx_out, rx_out) = channel();
        run(&mut parse(ex_16digit), channel().1, tx_out);
        assert_eq!(format!("{}", rx_out.recv().unwrap()).len(), 16);
        assert!(rx_out.recv().is_err());

        let ex_middle = "104,1125899906842624,99";
        let (tx_out, rx_out) = channel();
        run(&mut parse(ex_middle), channel().1, tx_out);
        assert_eq!(1125899906842624, rx_out.recv().unwrap());
        assert!(rx_out.recv().is_err());
    }
}
