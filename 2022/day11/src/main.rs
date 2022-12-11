use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    fn apply(&self, worry: u64) -> u64 {
        match self {
            Op::Add(other) => worry + other,
            Op::Mul(other) => worry * other,
            Op::Square => worry * worry,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    divisible_by: u64,
    true_throw_to: usize,
    false_throw_to: usize,
}

impl Monkey {
    fn throw_to(&self, worry: u64) -> usize {
        if worry % self.divisible_by == 0 {
            self.true_throw_to
        } else {
            self.false_throw_to
        }
    }
}

fn parse(puzzle_input: &str) -> Vec<Monkey> {
    const STARTING_ITEMS_PREFIX: &str = "  Starting items: ";
    const OPERATION_PREFIX: &str = "  Operation: new = old ";
    const OP_ADD_PREFIX: &str = "+ ";
    const OP_MUL_PREFIX: &str = "* ";
    const TEST_DIVISIBLE_PREFIX: &str = "  Test: divisible by ";
    const TRUE_THROW_TO_PREFIX: &str = "    If true: throw to monkey ";
    const FALSE_THROW_TO_PREFIX: &str = "    If false: throw to monkey ";
    puzzle_input.trim_end().split("\n\n").enumerate().map(|(i, monkey_input)| {
        let mut lines_it = monkey_input.lines();
        assert_eq!(lines_it.next().unwrap(), format!("Monkey {}:", i));
        let starting_items_str = lines_it.next().unwrap();
        assert!(starting_items_str.starts_with(STARTING_ITEMS_PREFIX));
        let items = starting_items_str[STARTING_ITEMS_PREFIX.len()..].split(", ")
            .map(|worry_str| worry_str.parse().unwrap())
            .collect();
        let op_str_full = lines_it.next().unwrap();
        assert!(op_str_full.starts_with(OPERATION_PREFIX));
        let op_str = &op_str_full[OPERATION_PREFIX.len()..];
        let op = if op_str == "* old" {
            Op::Square
        } else if op_str.starts_with(OP_ADD_PREFIX) {
            Op::Add(op_str[OP_ADD_PREFIX.len()..].parse().unwrap())
        } else if op_str.starts_with(OP_MUL_PREFIX) {
            Op::Mul(op_str[OP_MUL_PREFIX.len()..].parse().unwrap())
        } else {
            panic!();
        };
        let divisible_by_str = lines_it.next().unwrap();
        assert!(divisible_by_str.starts_with(TEST_DIVISIBLE_PREFIX));
        let divisible_by = divisible_by_str[TEST_DIVISIBLE_PREFIX.len()..].parse().unwrap();
        let true_throw_to_str = lines_it.next().unwrap();
        assert!(true_throw_to_str.starts_with(TRUE_THROW_TO_PREFIX));
        let true_throw_to = true_throw_to_str[TRUE_THROW_TO_PREFIX.len()..].parse().unwrap();
        assert_ne!(i, true_throw_to);
        let false_throw_to_str = lines_it.next().unwrap();
        assert!(false_throw_to_str.starts_with(FALSE_THROW_TO_PREFIX));
        let false_throw_to = false_throw_to_str[FALSE_THROW_TO_PREFIX.len()..].parse().unwrap();
        assert_ne!(i, false_throw_to);
        assert!(lines_it.next().is_none());
        Monkey { items, op, divisible_by, true_throw_to, false_throw_to }
    }).collect()
}

fn simulate(mut monkeys: Vec<Monkey>, num_rounds: usize, low_worry: bool) -> usize {
    let mut inspect_cnt = vec![0; monkeys.len()];
    let mut all_mod = 1;
    for monkey in &monkeys {
        all_mod *= monkey.divisible_by;
    }
    for _ in 0..num_rounds {
        for monkey_id in 0..monkeys.len() {
            let num_items = monkeys[monkey_id].items.len();
            inspect_cnt[monkey_id] += num_items;
            for item_id in 0..num_items {
                let worry = monkeys[monkey_id].items[item_id];
                let worry = monkeys[monkey_id].op.apply(worry);
                let worry = if low_worry { worry / 3 } else { worry % all_mod };
                let throw_to = monkeys[monkey_id].throw_to(worry);
                monkeys[throw_to].items.push(worry);
            }
            monkeys[monkey_id].items.clear();
        }
    }
    inspect_cnt.sort();
    inspect_cnt[inspect_cnt.len() - 1] * inspect_cnt[inspect_cnt.len() - 2]
}

fn part1(monkeys: Vec<Monkey>) -> usize {
    simulate(monkeys, 20, true)
}

fn part2(monkeys: Vec<Monkey>) -> usize {
    simulate(monkeys, 10000, false)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let monkeys = parse(&puzzle_input);
    println!("{}", part1(monkeys.clone()));
    println!("{}", part2(monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(EX)), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(EX)), 2713310158);
    }
}
