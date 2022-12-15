use std::cmp::Ordering;
use std::io::{self, Read};

enum Packet {
    List(Vec<Packet>),
    Int(u32),
}


impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Int(a),   Packet::Int(b))   => a.cmp(b),
            (Packet::List(_),  Packet::Int(b))   => self.cmp(&Packet::List(vec![Packet::Int(*b)])),
            (Packet::Int(a),   Packet::List(_))  => Packet::List(vec![Packet::Int(*a)]).cmp(other),
            (Packet::List(la), Packet::List(lb)) => {
                let mut a_it = la.iter();
                let mut b_it = lb.iter();
                loop {
                    match (a_it.next(), b_it.next()) {
                        (Some(a), Some(b)) => {
                            let cmp = a.cmp(b);
                            if cmp != Ordering::Equal {
                                return cmp
                            }
                        },
                        (Some(_), None)    => return Ordering::Greater,
                        (None,    Some(_)) => return Ordering::Less,
                        (None,    None)    => return Ordering::Equal,
                    }
                }
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Packet {}



impl Packet {
    fn parse(line: &str) -> Packet {
        let mut stack: Vec<Vec<Packet>> = Vec::new();
        let mut curr: Option<u32> = None;
        let mut ret: Option<Packet> = None;
        for ch in line.chars() {
            assert!(ret.is_none());
            match ch {
                '[' => {
                    assert!(curr.is_none());
                    stack.push(Vec::new());
                },
                ']' => {
                    let mut list = stack.pop().unwrap();
                    if let Some(i) = curr {
                        list.push(Packet::Int(i));
                    }
                    curr = None;
                    if stack.is_empty() {
                        ret = Some(Packet::List(list));
                    } else {
                        stack.last_mut().unwrap().push(Packet::List(list));
                    }
                },
                ',' => {
                    if let Some(i) = curr {
                        stack.last_mut().unwrap().push(Packet::Int(i));
                    } else {
                        // Already handled when parsing the ']'
                    }
                    curr = None;
                },
                '0'..='9' => {
                    if curr == None {
                        curr = Some(0);
                    }
                    *curr.as_mut().unwrap() *= 10;
                    *curr.as_mut().unwrap() += ch as u32 - '0' as u32;
                },
                _ => panic!(),

            }
        }
        if let Some(i) = curr {
            assert!(ret.is_none());
            ret = Some(Packet::Int(i));
        }
        ret.unwrap()
    }
}

fn parse(puzzle_input: &str) -> Vec<[Packet; 2]> {
    puzzle_input.split("\n\n").map(|pair_input| {
        let mut pair_lines = pair_input.lines();
        let left = Packet::parse(pair_lines.next().unwrap());
        let right = Packet::parse(pair_lines.next().unwrap());
        assert!(pair_lines.next().is_none());
        [left, right]
    }).collect()
}

fn part1(packet_pairs: &[[Packet; 2]]) -> usize {
    let mut sum = 0;
    for (i, [left, right]) in packet_pairs.iter().enumerate() {
        if left < right {
            sum += i + 1;
        }
    }
    sum
}

fn part2(packet_pairs: Vec<[Packet; 2]>) -> usize {
    let mut packets = Vec::with_capacity(2 * packet_pairs.len() + 2);
    for [left, right] in packet_pairs {
        packets.push(left);
        packets.push(right);
    }
    packets.sort();

    let pos1 = packets.binary_search(&Packet::parse("[[2]]")).unwrap_err() + 1;
    let pos2 = packets.binary_search(&Packet::parse("[[6]]")).unwrap_err() + 2;
    pos1 * pos2
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let packet_pairs = parse(&puzzle_input);
    println!("{}", part1(&packet_pairs));
    println!("{}", part2(packet_pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(EX)), 140);
    }
}
