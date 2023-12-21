use std::collections::HashMap;
use std::cmp::{min, max};
use std::io::{self, Read};
use std::ops::Range;

use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Category {
    X, M, A, S,
}

#[derive(Copy, Clone)]
enum Op {
    LT, GT,
}

enum Outcome {
    Accept, Reject, To(String),
}


struct Condition {
    category: Category,
    op: Op,
    val: u32,

}

struct Workflow {
    rules: Vec<(Condition, Outcome)>,
    default: Outcome,
}

type Part = HashMap<Category, u32>;

fn parse_condition(input: &str) -> Condition {
    let category = match &input[0..1] {
        "x" => Category::X,
        "m" => Category::M,
        "a" => Category::A,
        "s" => Category::S,
        _ => panic!(),
    };
    let op = match &input[1..2] {
        "<" => Op::LT,
        ">" => Op::GT,
        _ => panic!(),
    };
    let val = input[2..].parse().unwrap();
    Condition { category, op, val }
}

fn parse_outcome(input: &str) -> Outcome {
    match input {
        "A" => Outcome::Accept,
        "R" => Outcome::Reject,
        _ => Outcome::To(input.to_string()),
    }
}

fn parse_workflow(workflow_input: &str) -> (String, Workflow) {
    assert_eq!(&workflow_input[(workflow_input.len()-1)..workflow_input.len()], "}");
    let workflow_input = &workflow_input[..(workflow_input.len()-1)];
    let brace_idx = workflow_input.find("{").unwrap();
    let name = workflow_input[..brace_idx].to_string();
    let workflow_input = workflow_input[(brace_idx+1)..].to_string();
    let last_comma_idx = workflow_input.rfind(",").unwrap();
    let default = parse_outcome(&workflow_input[(last_comma_idx+1)..]);
    let workflow_input = &workflow_input[..last_comma_idx];
    let rules = workflow_input.split(',').map(|rule_input| {
        let mut sp_iter = rule_input.split(":");
        let condition = parse_condition(sp_iter.next().unwrap());
        let outcome = parse_outcome(sp_iter.next().unwrap());
        assert!(sp_iter.next().is_none());
        (condition, outcome)
    }).collect();

    (name, Workflow { rules, default})
}

fn parse_workflows(workflows_input: &str) -> HashMap<String, Workflow> {
    workflows_input.lines().map(|line| parse_workflow(line)).collect()
}

fn parse_parts(parts_input: &str) -> Vec<Part> {
    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    parts_input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let mut part = HashMap::new();
        part.insert(Category::X, cap[1].parse().unwrap());
        part.insert(Category::M, cap[2].parse().unwrap());
        part.insert(Category::A, cap[3].parse().unwrap());
        part.insert(Category::S, cap[4].parse().unwrap());
        part
    }).collect()
}

fn parse(puzzle_input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut blocks_iter = puzzle_input.split("\n\n");
    let workflows = parse_workflows(blocks_iter.next().unwrap());
    let parts = parse_parts(blocks_iter.next().unwrap());
    assert!(blocks_iter.next().is_none());
    (workflows, parts)
}

fn workflow_accepts(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut curr_name = "in".to_string();
    'outer: loop {
        let workflow = &workflows[&curr_name];
        for (condition, outcome) in &workflow.rules {
            let part_val = part[&condition.category];
            let matches_rule = match condition.op {
                Op::LT => part_val < condition.val,
                Op::GT => part_val > condition.val,
            };
            if matches_rule {
                match outcome {
                    Outcome::Accept => return true,
                    Outcome::Reject => return false,
                    Outcome::To(name) => {
                        curr_name = name.to_string();
                        continue 'outer;
                    }
                }
            }
        }
        match &workflow.default {
            Outcome::Accept => return true,
            Outcome::Reject => return false,
            Outcome::To(name) => curr_name = name.to_string(),
        }
    }
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &[Part]) -> u32 {
    let mut sum = 0;
    for part in parts {
        if workflow_accepts(workflows, part) {
            sum += part[&Category::X];
            sum += part[&Category::M];
            sum += part[&Category::A];
            sum += part[&Category::S];
        }
    }
    sum
}

// Returns (matching, not_matching)
fn split_range(range: Range<u32>, op: Op, val: u32) -> (Range<u32>, Range<u32>) {
    match op {
        Op::LT => (
            range.start..min(range.end, val),
            max(range.start, val)..range.end
        ),
        Op::GT => (
            max(range.start, val+1)..range.end,
            range.start..min(range.end, val+1)
        ),
    }
}

fn part2(workflows: &HashMap<String, Workflow>) -> u64 {
    let mut pending = Vec::new();
    pending.push({
        let mut combo = HashMap::new();
        combo.insert(Category::X, 1..4001);
        combo.insert(Category::M, 1..4001);
        combo.insert(Category::A, 1..4001);
        combo.insert(Category::S, 1..4001);
        (combo, "in".to_string())
    });
    let mut cnt = 0u64;
    'outer: while let Some((mut combo, name)) = pending.pop() {
        let workflow = &workflows[&name];
        for (condition, outcome) in &workflow.rules {
            let part_range = combo[&condition.category].clone();
            let (matching_range, non_matching_range) =
                split_range(part_range, condition.op, condition.val);
            if !matching_range.is_empty() {
                let mut matching_combo = combo.clone();
                matching_combo.insert(condition.category, matching_range);
                match outcome {
                    Outcome::Accept => {
                        let mut product = 1;
                        for r in matching_combo.values() {
                            product *= (r.end - r.start) as u64;
                        }
                        cnt += product;
                    }
                    Outcome::Reject => (),
                    Outcome::To(name) => {
                        pending.push((matching_combo, name.to_string()));
                    }
                }
            }
            if non_matching_range.is_empty() {
                continue 'outer;
            } else {
                combo.insert(condition.category, non_matching_range);
            }
        }
        match &workflow.default {
            Outcome::Accept => {
                let mut product = 1;
                for r in combo.values() {
                    product *= (r.end - r.start) as u64;
                }
                cnt += product;
            }
            Outcome::Reject => (),
            Outcome::To(name) => {
                pending.push((combo, name.to_string()));
            }
        }
    }
    cnt
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (workflows, parts) = parse(&puzzle_input);
    println!("{}", part1(&workflows, &parts));
    println!("{}", part2(&workflows));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        let (workflows, parts) = parse(EX);
        assert_eq!(part1(&workflows, &parts), 19114);
    }

    #[test]
    fn test_part2() {
        let (workflows, _parts) = parse(EX);
        assert_eq!(part2(&workflows), 167409079868000);
    }
}
