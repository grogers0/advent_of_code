use std::io::{self, Read};
use std::collections::HashMap;

enum Op {
    OR, AND, XOR
}
struct Gate {
    in1: String,
    in2: String,
    op: Op,
    out: String,
}

fn parse_inputs(inputs_str: &str) -> HashMap<String, bool> {
    inputs_str.lines().map(|line| {
        let mut sp = line.split(": ");
        let wire = sp.next().unwrap().to_string();
        let val = match sp.next().unwrap() {
            "0" => false,
            "1" => true,
            _ => panic!(),
        };
        assert!(sp.next().is_none());
        (wire, val)
    }).collect()
}

fn parse_gates(gates_input: &str) -> Vec<Gate> {
    gates_input.lines().map(|line| {
        let mut sp = line.split(" ");
        let in1 = sp.next().unwrap().to_string();
        let op = match sp.next().unwrap() {
            "OR"  => Op::OR,
            "AND" => Op::AND,
            "XOR" => Op::XOR,
            _ => panic!(),
        };
        let in2 = sp.next().unwrap().to_string();
        assert_eq!(sp.next().unwrap(), "->");
        let out = sp.next().unwrap().to_string();
        assert!(sp.next().is_none());
        Gate { in1, in2, op, out }
    }).collect()
}


fn parse(puzzle_input: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let mut sp = puzzle_input.split("\n\n");
    let inputs = parse_inputs(sp.next().unwrap());
    let gates = parse_gates(sp.next().unwrap());
    (inputs, gates)
}

fn part1(inputs: &HashMap<String, bool>, gates: &[Gate]) -> u64 {
    let mut out_to_gate = HashMap::new();
    let mut in_to_out = HashMap::new();
    for gate in gates {
        out_to_gate.insert(gate.out.clone(), gate);
        for input in [&gate.in1, &gate.in2] {
            in_to_out.entry(input.clone())
                .and_modify(|outputs: &mut Vec<String>| outputs.push(gate.out.clone()))
                .or_insert(vec![gate.out.clone()]);
        }
    }
    let mut wires = HashMap::new();
    for (input, val) in inputs {
        wires.insert(input.to_string(), *val);
    }
    let mut stack = gates.iter().map(|gate| gate.out.clone()).collect::<Vec<_>>();
    while let Some(out) = stack.pop() {
        let gate = out_to_gate[&out];
        if let Some(outval1) = wires.get(&out) {
            continue;
        }
        if let Some(&inval1) = wires.get(&gate.in1) {
            if let Some(&inval2) = wires.get(&gate.in2) {
                let outval = match gate.op {
                    Op::OR  => inval1 || inval2,
                    Op::AND => inval1 && inval2,
                    Op::XOR => inval1 != inval2,
                };
                wires.insert(out.clone(), outval);
                // Possibly recalculate since we have the wire set now
                if let Some(outputs) = in_to_out.get(&out) {
                    for out2 in outputs {
                        stack.push(out2.clone());
                    }
                }
            }
        }
    }

    let mut ret = 0;
    for i in 0..64 {
        if let Some(&true) = wires.get(&format!("z{i:02}")) {
            ret |= 1 << i;
        }
    }
    ret
}

fn part2(puzzle_input: &str) -> &str {
    // Each section of the adder looks like this:
    //
    // x02 AND y02 -> a02
    // x02 XOR y02 -> b02
    // b01 AND d01 -> c02
    // a01 OR c02 -> d02
    // b02 XOR d02 -> z02

    // y03 AND x03 -> a03
    // x03 XOR y03 -> b03
    // b02 AND d02 -> c03
    // a02 OR c03 -> d03
    // b03 XOR d03 -> z03
    // 
    // So anything that doesn't match that pattern you can find the correct switch for.
    // TODO - I did this manually by hand (see notes.txt) - maybe I'll come back and implement the
    // algorithm in code.
    "hqh,mmk,pvb,qdq,vkq,z11,z24,z38"
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (inputs, gates) = parse(&puzzle_input);
    println!("{}", part1(&inputs, &gates));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EX2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        let (inputs, gates) = parse(EX1);
        assert_eq!(part1(&inputs, &gates), 4);
        let (inputs, gates) = parse(EX2);
        assert_eq!(part1(&inputs, &gates), 2024);
    }
}
