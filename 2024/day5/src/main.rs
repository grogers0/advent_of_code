use std::io::{self, Read};
use std::collections::HashMap;

struct ParsedInput {
    rules: Vec<[u64; 2]>,
    pages: Vec<Vec<u64>>,
}

fn parse_rules(rules_input: &str) -> Vec<[u64; 2]> {
    rules_input.lines().map(|line| {
        let mut sp = line.split("|");
        let x = sp.next().unwrap().parse::<u64>().unwrap();
        let y = sp.next().unwrap().parse::<u64>().unwrap();
        assert!(sp.next().is_none());
        [x, y]
    }).collect()
}

fn parse_pages(pages_input: &str) -> Vec<Vec<u64>> {
    pages_input.lines().map(|line| {
        line.split(",").map(|s| s.parse::<u64>().unwrap()).collect()
    }).collect()
}

fn parse(puzzle_input: &str) -> ParsedInput {
    let mut sp = puzzle_input.split("\n\n");
    let rules = parse_rules(sp.next().unwrap());
    let pages = parse_pages(sp.next().unwrap());
    assert!(sp.next().is_none());
    ParsedInput { rules, pages }
}

fn is_correct_order(rules: &[[u64; 2]], pages: &[u64]) -> bool {
    let mut page_order = HashMap::new();
    for (i, v) in pages.iter().enumerate() {
        let prior = page_order.insert(v, i);
        assert!(prior.is_none());
    }
    for &[a, b] in rules {
        if let Some(a_i) = page_order.get(&a) {
            if let Some(b_i) = page_order.get(&b) {
                if a_i > b_i { return false; }
            }
        }
    }
    true
}

fn part1(parsed_input: &ParsedInput) -> u64 {
    let mut sum = 0;
    for pages in &parsed_input.pages {
        if is_correct_order(&parsed_input.rules, pages) {
            sum += pages[pages.len() / 2];
        }
    }
    sum
}

fn find_correct_order(rules: &[[u64; 2]], pages_in: &[u64]) -> Vec<u64> {
    fn find_correct_order_recur(rules: &[[u64; 2]], pages_in: &[u64], pages_curr: &mut Vec<u64>) ->
        Option<Vec<u64>> {
        if !is_correct_order(rules, &pages_curr) { return None; }
        if pages_in.is_empty() { return Some(pages_curr.clone()); }
        pages_curr.push(pages_in[0]);
        let pages_in = &pages_in[1..];
        if let Some(ret) = find_correct_order_recur(rules, pages_in, pages_curr) {
            return Some(ret);
        }
        let mut i = pages_curr.len() - 1;
        while i > 0 {
            pages_curr.swap(i - 1, i);
            i -= 1;
            if let Some(ret) = find_correct_order_recur(rules, pages_in, pages_curr) {
                return Some(ret);
            }
        }
        None
    }


    let mut pages_out = Vec::with_capacity(pages_in.len());
    find_correct_order_recur(rules, pages_in, &mut pages_out).unwrap()
}

fn part2(parsed_input: &ParsedInput) -> u64 {
    let mut sum = 0;
    for pages in &parsed_input.pages {
        if !is_correct_order(&parsed_input.rules, pages) {
            let pages = find_correct_order(&parsed_input.rules, pages);
            sum += pages[pages.len() / 2];
        }
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let parsed_input = parse(&puzzle_input);
    println!("{}", part1(&parsed_input));
    println!("{}", part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 123);
    }
}
