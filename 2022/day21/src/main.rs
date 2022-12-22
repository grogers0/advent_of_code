use std::collections::HashMap;
use std::io::{self, Read};

enum Op {
    Add, Sub, Mul, Div
}

enum Yell {
    Num(i64),
    Math(String, String, Op),
}

// Our solutions to both parts rely on the structure of the input being such that any monkey is
// referenced by at most one other monkey. This means we don't have to solve generic symbolic
// execution or bother to memoize.
fn assert_single_references(monkeys: &HashMap<String, Yell>) {
    let mut refs = HashMap::new();
    for (_, yell) in monkeys {
        let names = match yell {
            Yell::Num(_) => continue,
            Yell::Math(a, b, _) => [a, b],
        };
        for name in names {
            *refs.entry(name).or_insert(0) += 1;
        }
    }
    for (_, cnt) in refs {
        assert_eq!(cnt, 1);
    }
}

fn parse(puzzle_input: &str) -> HashMap<String, Yell> {
    let monkeys = puzzle_input.trim_end().lines().map(|line| {
        let name = line[0..4].to_string();
        assert_eq!(&line[4..6], ": ");
        let line = &line[6..];
        let yell = if let Ok(num) = line.parse() {
            Yell::Num(num)
        } else {
            assert_eq!(line.len(), 11);
            let name1 = line[0..4].to_string();
            let name2 = line[7..11].to_string();
            let op = match &line[4..7] {
                " + " => Op::Add,
                " - " => Op::Sub,
                " * " => Op::Mul,
                " / " => Op::Div,
                _ => panic!(),
            };
            Yell::Math(name1, name2, op)
        };
        (name, yell)

    }).collect();
    assert_single_references(&monkeys);
    monkeys
}

fn calc_yell(monkeys: &HashMap<String, Yell>, monkey: &str) -> i64 {
    match &monkeys[monkey] {
        Yell::Num(num) => *num,
        Yell::Math(name1, name2, op) => {
            let val1 = calc_yell(monkeys, name1);
            let val2 = calc_yell(monkeys, name2);
            match op {
                Op::Add => val1 + val2,
                Op::Sub => val1 - val2,
                Op::Mul => val1 * val2,
                Op::Div => {
                    debug_assert_eq!(val1 % val2, 0);
                    val1 / val2
                }
            }
        }
    }
}

fn part1(monkeys: &HashMap<String, Yell>) -> i64 {
    calc_yell(monkeys, "root")
}

fn part2(monkeys: &HashMap<String, Yell>) -> i64 {
    fn calc_refs_human(monkeys: &HashMap<String, Yell>, root: &str, memo: &mut HashMap<String, bool>) -> bool {
        if memo.contains_key(root) { return memo[root] }
        let ret = match &monkeys[root] {
            Yell::Num(_) => false,
            Yell::Math(name1, name2, _) => {
                // No short circuit to memoize the full tree
                let refs1 = calc_refs_human(monkeys, name1, memo);
                let refs2 = calc_refs_human(monkeys, name2, memo);
                refs1 || refs2
            }
        };
        let ret = ret || root == "humn";
        memo.insert(root.to_string(), ret);
        ret
    }

    let mut refs_human = HashMap::new();
    calc_refs_human(monkeys, "root", &mut refs_human);

    let Yell::Math(left, right, _) = &monkeys["root"] else { panic!() };
    let (mut val, mut curr) = if refs_human[left] {
        (calc_yell(monkeys, right), left)
    } else {
        (calc_yell(monkeys, left), right)
    };
    while curr != "humn" {
        debug_assert!(refs_human[curr]);
        let Yell::Math(left, right, op) = &monkeys[curr] else { panic!() };
        let other;
        if refs_human[left] {
            // val = humn OP other
            curr = left;
            other = calc_yell(monkeys, right);
            match op {
                Op::Add => val -= other,
                Op::Sub => val += other,
                Op::Mul => {
                    debug_assert_eq!(val % other, 0);
                    val /= other;
                },
                Op::Div => val *= other,
            }
        } else {
            // val = other OP humn
            curr = right;
            other = calc_yell(monkeys, left);
            match op {
                Op::Add => val -= other,
                Op::Sub => val = other - val,
                Op::Mul => {
                    debug_assert_eq!(val % other, 0);
                    val /= other;
                },
                Op::Div => {
                    debug_assert_eq!(other % val, 0);
                    val = other / val;
                },
            }
        }
    }
    val
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let monkeys = parse(&puzzle_input);
    println!("{}", part1(&monkeys));
    println!("{}", part2(&monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 301);
    }
}
