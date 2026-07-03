use std::io::{self, Read};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn parse(puzzle_input: &str) -> HashMap<String, HashSet<String>> {
    let connections_raw: Vec<_> = puzzle_input.lines().map(|line| {
        let mut sp = line.split("-");
        let a = sp.next().unwrap().to_string();
        let b = sp.next().unwrap().to_string();
        assert!(sp.next().is_none());
        (a, b)
    }).collect();

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for (a, b) in connections_raw {
        connections.entry(a.clone())
            .and_modify(|set| { set.insert(b.clone()); })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(b.clone());
                set
            });
        connections.entry(b.clone())
            .and_modify(|set| { set.insert(a.clone()); })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(a.clone());
                set
            });
    }
    connections
}

fn part1(connections: &HashMap<String, HashSet<String>>) -> usize {
    let mut cnt = 0;
    let mut seen = HashSet::new();
    for (a, others_a) in connections {
        for b in others_a {
            if a == b { continue; }
            for c in &connections[b] {
                if a == c || b == c { continue; }
                if !connections[c].contains(a) { continue; }
                if !seen.insert(BTreeSet::from([a.clone(), b.clone(), c.clone()])) { continue; }

                if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

fn part2(connections: &HashMap<String, HashSet<String>>) -> String {
    let mut seen = HashSet::new();
    let mut best = BTreeSet::new();
    let mut deque = VecDeque::new();
    for (x, _) in connections {
        deque.push_back(BTreeSet::from([x.clone()]));
    }

    while let Some(set) = deque.pop_front() {
        if !seen.insert(set.clone()) { continue; }
        if set.len() > best.len() { best = set.clone(); }

        for x in &connections[set.iter().next().unwrap()] {
            if set.contains(x) { continue; }
            let others = &connections[x];
            if set.iter().all(|y| others.contains(y)) {
                let mut set2 = set.clone();
                set2.insert(x.clone());
                deque.push_back(set2);
            }
        }
    }

    best.iter().cloned().collect::<Vec<_>>().join(",")
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let connections = parse(&puzzle_input);
    println!("{}", part1(&connections));
    println!("{}", part2(&connections));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(&parse(EX)), "co,de,ka,ta");
    }
}
