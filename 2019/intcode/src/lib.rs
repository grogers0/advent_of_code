type Int = i32;
type Mem = Vec<Int>;

pub fn parse(mem_str: &str) -> Mem {
    mem_str.trim().split(",")
        .map(|chunk| chunk.parse().unwrap())
        .collect()
}

enum ParamMode {
    Position,
    Immediate
}

fn param_mode(mem: &Mem, pc: usize, param_num: usize) -> ParamMode {
    match mem[pc] / Int::pow(10, param_num as u32 + 1) % 10 {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        mode => unimplemented!("unknown parameter mode {}", mode)
    }
}

fn param(mem: &Mem, pc: usize, param_num: usize) -> Int {
    match param_mode(mem, pc, param_num) {
        ParamMode::Position => mem[mem[pc + param_num] as usize],
        ParamMode::Immediate => mem[pc + param_num]
    }
}

fn param_mut(mem: &mut Mem, pc: usize, param_num: usize) -> &mut Int {
    match param_mode(mem, pc, param_num) {
        ParamMode::Position => {
            let offset = mem[pc + param_num] as usize;
            &mut mem[offset]
        },
        ParamMode::Immediate => panic!("parameter {} of instruction {} at pc {} is a write in immediate mode",
            param_num, inst(mem, pc), pc)
    }
}

fn inst(mem: &Mem, pc: usize) -> Int {
    mem[pc] % 100
}

pub fn run(mem: &mut Mem, input: Option<Int>) -> Vec<Int> {
    let mut pc = 0;
    let mut output = Vec::new();
    loop {
        match inst(mem, pc) {
            1 => { // add
                *param_mut(mem, pc, 3) = param(mem, pc, 1) + param(mem, pc, 2);
                pc += 4;
            },
            2 => { // mul
                *param_mut(mem, pc, 3) = param(mem, pc, 1) * param(mem, pc, 2);
                pc += 4;
            },
            3 => { // read input
                *param_mut(mem, pc, 1) = input.expect("no input given");
                pc += 2;
            },
            4 => { // write output
                output.push(param(mem, pc, 1));
                pc += 2;
            },
            5 => { // jump if nonzero
                if param(mem, pc, 1) != 0 {
                    pc = param(mem, pc, 2) as usize;
                } else {
                    pc += 3;
                }
            },
            6 => { // jump if zero
                if param(mem, pc, 1) == 0 {
                    pc = param(mem, pc, 2) as usize;
                } else {
                    pc += 3;
                }
            },
            7 => { // less than
                *param_mut(mem, pc, 3) = if param(mem, pc, 1) < param(mem, pc, 2) { 1 } else { 0 };
                pc += 4;
            },
            8 => { // equals
                *param_mut(mem, pc, 3) = if param(mem, pc, 1) == param(mem, pc, 2) { 1 } else { 0 };
                pc += 4;
            },
            99 => break, // halt
            opcode => unimplemented!("unknown opcode {}", opcode)
        }
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    fn run_no_io(mem_str: &str) -> Mem {
        let mut mem = parse(mem_str);
        let output = run(&mut mem, None);
        assert_eq!(Vec::<Int>::new(), output);
        mem
    }

    fn run_single_io(mem_str: &str, input: Int) -> Int {
        let mut mem = parse(mem_str);
        let output = run(&mut mem, Some(input));
        assert_eq!(1, output.len());
        output[0]
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
}
