use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum Loc {
    Val(i64),
    Reg(String)
}

impl From<&str> for Loc {
    fn from(s: &str) -> Loc {
        if let Ok(val) = s.parse() {
            Loc::Val(val)
        } else {
            Loc::Reg(s.to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Op {
    Cpy(Loc, Loc),
    Inc(String),
    Dec(String),
    Jnz(Loc, Loc),
    Tgl(String)
}

pub fn parse_ops(input: &str) -> Vec<Op> {
    input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "cpy" => Op::Cpy(Loc::from(tokens.next().unwrap()), Loc::from(tokens.next().unwrap())),
            "inc" => Op::Inc(tokens.next().unwrap().to_string()),
            "dec" => Op::Dec(tokens.next().unwrap().to_string()),
            "jnz" => Op::Jnz(Loc::from(tokens.next().unwrap()), Loc::from(tokens.next().unwrap())),
            "tgl" => Op::Tgl(tokens.next().unwrap().to_string()),
            _ => unreachable!()
        }
    })
    .collect()
}

fn lookup(loc: &Loc, registers: &BTreeMap<String, i64>) -> i64 {
    match loc {
        Loc::Val(x) => *x,
        Loc::Reg(x) => *registers.get(x).unwrap_or(&0)
    }
}

pub fn execute_op(program: &mut Vec<Op>, pc: &mut i64, registers: &mut BTreeMap<String, i64>) {
    match &program[*pc as usize] {
        Op::Cpy(_, Loc::Val(_)) => (), // Invalid, skip
        Op::Cpy(x, Loc::Reg(y)) => { registers.insert(y.clone(), lookup(x, registers)); },
        Op::Inc(x) => { registers.entry(x.to_string()).and_modify(|v| *v += 1).or_insert(1); },
        Op::Dec(x) => { registers.entry(x.to_string()).and_modify(|v| *v -= 1).or_insert(-1); },
        Op::Jnz(x, y) => if lookup(x, registers) != 0 { *pc += lookup(y, registers) - 1; },
        Op::Tgl(x) => {
            let off = *pc + *registers.get(x).unwrap_or(&0);
            if off >= 0 && off < program.len() as i64 {
                program[off as usize] = match &program[off as usize] {
                    Op::Cpy(x, y) => Op::Jnz(x.clone(), y.clone()),
                    Op::Inc(x) => Op::Dec(x.clone()),
                    Op::Dec(x) => Op::Inc(x.clone()),
                    Op::Jnz(x, y) => Op::Cpy(x.clone(), y.clone()),
                    Op::Tgl(x) => Op::Inc(x.clone())
                };
            }
        }
    }
    *pc += 1;
}

pub fn execute_with_initial_state(input: &str, f: fn(&mut BTreeMap<String, i64>)) -> i64 {
    let mut program = parse_ops(input);
    let mut pc = 0i64;
    let mut registers = BTreeMap::new();
    f(&mut registers);
    while pc >= 0 && pc < program.len() as i64 {
        execute_op(&mut program, &mut pc, &mut registers);
    }
    *registers.get("a").unwrap_or(&0)
}
