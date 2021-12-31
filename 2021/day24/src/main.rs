use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Operand {
    Var(char),
    Num(i64),
}

impl Operand {
    fn parse(s: &str) -> Operand {
        match s {
            "w" | "x" | "y" | "z" => Operand::Var(s.chars().next().unwrap()),
            _ => Operand::Num(s.parse().unwrap()),
        }
    }

    fn get(&self, mem: &Mem) -> i64 {
        match self {
            Operand::Var(a) => mem.get(*a),
            Operand::Num(x) => *x,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Instruction {
    Inp(char),
    Add(char, Operand),
    Mul(char, Operand),
    Div(char, Operand),
    Mod(char, Operand),
    Eql(char, Operand),
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let mut it = line.split(" ");
        let op_type = it.next().unwrap();
        let a = it.next().unwrap().chars().next().unwrap();
        let b = it.next().map(Operand::parse);
        assert!(it.next().is_none());
        match op_type {
            "inp" => Instruction::Inp(a),
            "add" => Instruction::Add(a, b.unwrap()),
            "mul" => Instruction::Mul(a, b.unwrap()),
            "div" => Instruction::Div(a, b.unwrap()),
            "mod" => Instruction::Mod(a, b.unwrap()),
            "eql" => Instruction::Eql(a, b.unwrap()),
            _ => panic!(),
        }
    }
}

fn parse(puzzle_input: &str) -> Vec<Instruction> {
    puzzle_input.lines().map(Instruction::parse).collect()
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Mem {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Mem {
    fn new() -> Self {
        Self { w: 0, x: 0, y: 0, z: 0 }
    }

    fn get(&self, ch: char) -> i64 {
        match ch {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!(),
        }
    }

    fn ref_mut(&mut self, ch: char) -> &mut i64 {
        match ch {
            'w' => &mut self.w,
            'x' => &mut self.x,
            'y' => &mut self.y,
            'z' => &mut self.z,
            _ => panic!(),
        }
    }
}

fn exec_until_inp(program: &[Instruction], mem: &mut Mem, pc: &mut usize) {
    while *pc < program.len() {
        match program[*pc] {
            Instruction::Inp(_) => return,
            Instruction::Add(a, b) => *mem.ref_mut(a) += b.get(mem),
            Instruction::Mul(a, b) => *mem.ref_mut(a) *= b.get(mem),
            Instruction::Div(a, b) => *mem.ref_mut(a) /= b.get(mem),
            Instruction::Mod(a, b) => *mem.ref_mut(a) %= b.get(mem),
            Instruction::Eql(a, b) => {
                let eql = mem.get(a) == b.get(mem);
                *mem.ref_mut(a) = if eql { 1 } else { 0 };
            },
        };
        *pc += 1;
    }
}

// NOTE - this is pretty slow, like 30s in release mode. Not sure if there's another way to exploit
// symmetry in the solution other than identical mem/pc states
fn find_model_num(program: &[Instruction], mut mem: Mem, mut pc: usize, memo: &mut HashSet<(Mem, usize)>, largest: bool) -> Option<Vec<i64>> {
    exec_until_inp(program, &mut mem, &mut pc);
    if pc >= program.len() {
        if mem.get('z') == 0 {
            Some(vec![])
        } else {
            None
        }
    } else if let Instruction::Inp(a) = program[pc] {
        if !memo.insert((mem.clone(), pc)) { return None }
        pc += 1;
        let iter: Box<dyn Iterator<Item = i64>> = if largest {
            Box::new((1..10).rev())
        } else {
            Box::new(1..10)
        };
        for inp in iter {
            let mut mem2 = mem.clone();
            *mem2.ref_mut(a) = inp;
            if let Some(mut ret) = find_model_num(program, mem2, pc, memo, largest) {
                ret.push(inp);
                return Some(ret);
            }
        }
        None
    } else {
        unreachable!()
    }
}

fn inputs_rev_to_model_num(inputs_rev: Vec<i64>) -> i64 {
    let mut sum = 0;
    for x in inputs_rev.iter().copied().rev() {
        sum = sum * 10 + x;
    }
    sum
}

fn part1(puzzle_input: &str) -> i64 {
    let program = parse(puzzle_input);
    let inputs_rev = find_model_num(&program, Mem::new(), 0,
        &mut HashSet::new(), true).unwrap();
    inputs_rev_to_model_num(inputs_rev)
}

fn part2(puzzle_input: &str) -> i64 {
    let program = parse(puzzle_input);
    let inputs_rev = find_model_num(&program, Mem::new(), 0,
        &mut HashSet::new(), false).unwrap();
    inputs_rev_to_model_num(inputs_rev)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}
