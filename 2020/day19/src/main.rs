use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read};

#[derive(Clone, Hash, Eq, PartialEq)]
enum Rule {
    Char(char),
    Ref(usize),
    Seq(Vec<Rule>),
    Alt(Vec<Rule>)
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Char(ch) => write!(f, "\"{}\"", ch),
            Rule::Ref(id) => write!(f, "{}", id),
            Rule::Seq(rules) => {
                for (i, rule) in rules.iter().enumerate() {
                    if i > 0 { write!(f, " ")?; }
                    write!(f, "{}", rule)?;
                }
                Ok(())
            },
            Rule::Alt(rules) => {
                for (i, rule) in rules.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", rule)?;
                }
                Ok(())
            }
        }
    }
}

struct DisplayRules<'a>(&'a [Rule]);
impl <'a> fmt::Display for DisplayRules<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, rule) in self.0.iter().enumerate() {
            write!(f, "{}: {}\n", i, rule)?;
        }
        Ok(())
    }
}


fn parse(puzzle_input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    fn parse_rule(s: &str) -> Rule {
        if s.contains(" | ") {
            Rule::Alt(s.split(" | ").map(|part| parse_rule(part)).collect())
        } else if s.contains(" ") {
            Rule::Seq(s.split(" ").map(|part| parse_rule(part)).collect())
        } else if s.starts_with("\"") && s.ends_with("\"") {
            Rule::Char(s.chars().skip(1).next().unwrap())
        } else {
            Rule::Ref(s.parse().unwrap())
        }
    }

    fn parse_rule_line(line: &str) -> (usize, Rule) {
        let mut parts = line.split(": ");
        let id = parts.next().unwrap().parse().unwrap();
        let rule = parse_rule(parts.next().unwrap());
        assert!(parts.next().is_none());
        (id, rule)
    }

    fn parse_rules(s: &str) -> HashMap<usize, Rule> {
        s.lines().map(|line| parse_rule_line(line)).collect()
    }

    let mut parts = puzzle_input.split("\n\n");
    let rules = parse_rules(parts.next().unwrap());
    let messages = parts.next().unwrap().lines().map(|line| line.to_string()).collect();
    assert!(parts.next().is_none());
    (rules, messages)
}

// This takes ~30 seconds, maybe there's a faster way to do this based on the structure of the
// rules given
fn matches(rules: &HashMap<usize, Rule>, rule: &Rule, s: &str) -> bool {
    fn matches_memo(rules: &HashMap<usize, Rule>,
        rule: &Rule, s: &str,
        memo: &mut HashMap<(Rule, String), bool>) -> bool {
        if let Some(ret) = memo.get(&(rule.clone(), s.to_string())) { return *ret; }
        let ret = match rule {
            Rule::Char(ch) => ch.to_string() == s,
            Rule::Ref(ref_id) => matches_memo(rules, &rules[ref_id], s, memo),
            Rule::Seq(seq_rules) => {
                if seq_rules.is_empty() {
                    s.is_empty()
                } else {
                    let first_rule = &seq_rules[0];
                    let remaining_rules = Rule::Seq(seq_rules[1..].iter().cloned().collect());
                    (0..=s.len()).any(|i| matches_memo(rules, first_rule, &s[0..i], memo)
                            && matches_memo(rules, &remaining_rules, &s[i..], memo))
                }
            },
            Rule::Alt(alt_rules) => alt_rules.iter().any(|r| matches_memo(rules, r, s, memo))
        };
        memo.insert((rule.clone(), s.to_string()), ret);
        ret
    }
    let mut memo = HashMap::new();
    matches_memo(rules, rule, s, &mut memo)
}

fn part1(puzzle_input: &str) -> usize {
    let (rules, messages) = parse(&puzzle_input);
    messages.iter().filter(|m| matches(&rules, &rules[&0], &m)).count()
}

fn part2(puzzle_input: &str) -> usize {
    let (mut rules, messages) = parse(&puzzle_input);
    rules.insert(8, Rule::Alt(vec![
            Rule::Ref(42),
            Rule::Seq(vec![Rule::Ref(42), Rule::Ref(8)])]));
    rules.insert(11, Rule::Alt(vec![
            Rule::Seq(vec![Rule::Ref(42), Rule::Ref(31)]),
            Rule::Seq(vec![Rule::Ref(42), Rule::Ref(11), Rule::Ref(31)])]));
    messages.iter().filter(|m| matches(&rules, &rules[&0], &m)).count()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    const EX2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(EX1));
        assert_eq!(3, part1(EX2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2(EX2));
    }
}
