#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Addr, Addi, // rC = rA + (r/v)B
    Mulr, Muli, // rC = rA * (r/v)B
    Banr, Bani, // rC = rA & (r/v)B
    Borr, Bori, // rC = rA | (r/v)B
    Setr, Seti, // rC = (r/v)A // B ignored
    Gtir, Gtri, Gtrr, // if (r/v)A > (r/v)B { rC = 1 } else { rC = 0 }
    Eqir, Eqri, Eqrr, // if (r/v)A == (r/v)B { rC = 1 } else { rC = 0 }
    // Bonus instructions needed to optimize
    Divr // rC = rA / rB
}

pub fn execute_op(registers: &mut [usize], op: Op, a: usize, b: usize, c: usize) {
    match op {
        Op::Addr => registers[c] = registers[a] + registers[b],
        Op::Addi => registers[c] = registers[a] + b,
        Op::Mulr => registers[c] = registers[a] * registers[b],
        Op::Muli => registers[c] = registers[a] * b,
        Op::Banr => registers[c] = registers[a] & registers[b],
        Op::Bani => registers[c] = registers[a] & b,
        Op::Borr => registers[c] = registers[a] | registers[b],
        Op::Bori => registers[c] = registers[a] | b,
        Op::Setr => registers[c] = registers[a],
        Op::Seti => registers[c] = a,
        Op::Gtir => registers[c] = if a > registers[b] { 1 } else { 0 },
        Op::Gtri => registers[c] = if registers[a] > b { 1 } else { 0 },
        Op::Gtrr => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
        Op::Eqir => registers[c] = if a == registers[b] { 1 } else { 0 },
        Op::Eqri => registers[c] = if registers[a] == b { 1 } else { 0 },
        Op::Eqrr => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        Op::Divr => registers[c] = registers[a] / registers[b]
    }
}
