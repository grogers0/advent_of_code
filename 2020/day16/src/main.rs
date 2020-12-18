use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::ops::RangeInclusive;

type Rules = HashMap<String, Vec<RangeInclusive<u32>>>;
type Ticket = Vec<u32>;

fn parse(puzzle_input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    enum Mode {
        Rules,
        YourTicketHeader,
        YourTicketValue,
        NearbyTicketsHeader,
        NearbyTicketValues
    }

    fn parse_ticket(ticket: &str) -> Ticket {
        ticket.split(',').map(|n| n.parse().unwrap()).collect()
    }

    let mut mode = Mode::Rules;
    let mut rules = Rules::new();
    let mut your_ticket = Ticket::new();
    let mut nearby_tickets = Vec::new();
    for line in puzzle_input.lines() {
        match mode {
            Mode::Rules => {
                if line == "" {
                    mode = Mode::YourTicketHeader;
                } else {
                    let mut colon_parts = line.split(": ");
                    let field = colon_parts.next().unwrap().to_string();
                    let ranges = colon_parts.next().unwrap().split(" or ")
                        .map(|range_str| {
                            let mut range_parts = range_str.split("-");
                            let begin = range_parts.next().unwrap().parse().unwrap();
                            let end = range_parts.next().unwrap().parse().unwrap();
                            assert!(range_parts.next().is_none());
                            begin ..= end
                        }).collect();
                    assert!(colon_parts.next().is_none());
                    rules.insert(field, ranges);
                }
            },
            Mode::YourTicketHeader => {
                assert_eq!(line, "your ticket:");
                mode = Mode::YourTicketValue;
            },
            Mode::YourTicketValue => {
                mode = Mode::NearbyTicketsHeader;
                your_ticket = parse_ticket(line);
            },
            Mode::NearbyTicketsHeader => {
                match line {
                    "" => (),
                    "nearby tickets:" => mode = Mode::NearbyTicketValues,
                    _ => panic!()
                }
            },
            Mode::NearbyTicketValues => {
                nearby_tickets.push(parse_ticket(line));
            }
        }
    }
    (rules, your_ticket, nearby_tickets)
}

fn valid_for_ranges(ranges: &Vec<RangeInclusive<u32>>, val: u32) -> bool {
    ranges.iter().any(|range| range.contains(&val))
}
fn valid_for_some_rule(rules: &Rules, val: u32) -> bool {
    rules.values().any(|ranges| valid_for_ranges(ranges, val))
}

fn part1(rules: &Rules, nearby_tickets: &Vec<Ticket>) -> u32 {
    nearby_tickets.iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|&&val| !valid_for_some_rule(rules, val))
        .sum()
}

fn part2(rules: &Rules, your_ticket: &Ticket, nearby_tickets: &Vec<Ticket>) -> u64 {
    let nearby_tickets: Vec<_> = nearby_tickets.iter()
        .filter(|&ticket| ticket.iter().all(|&val| valid_for_some_rule(rules, val)))
        .cloned()
        .collect();

    let mut allowed_offsets: Vec<(&str, HashSet<usize>)> = rules.iter()
        .map(|(field, ranges)| {
            let offsets: HashSet<usize> = (0..rules.len())
                .filter(|&i| nearby_tickets.iter().all(|ticket| valid_for_ranges(ranges, ticket[i])))
                .collect();
            (field.as_str(), offsets)
        }).collect();
    allowed_offsets.sort_by_key(|(_, offsets)| offsets.len());
    let mut used_offsets: HashSet<usize> = HashSet::new();
    let mut correct_offsets: HashMap<&str, usize> = HashMap::new();
    for (field, offsets) in allowed_offsets {
        let offset = *offsets.iter().filter(|&i| !used_offsets.contains(i)).next().unwrap();
        used_offsets.insert(offset);
        correct_offsets.insert(field, offset);
    }

    correct_offsets.iter()
        .filter(|(field, _)| field.starts_with("departure"))
        .map(|(_, offset)| your_ticket[*offset] as u64)
        .product()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let (rules, your_ticket, nearby_tickets) = parse(&puzzle_input);

    println!("{}", part1(&rules, &nearby_tickets));
    println!("{}", part2(&rules, &your_ticket, &nearby_tickets));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const EX2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_part1() {
        let (rules, _your_ticket, nearby_tickets) = parse(EX1);
        assert_eq!(71, part1(&rules, &nearby_tickets));
    }

    #[test]
    fn test_part2() {
        let (rules, your_ticket, nearby_tickets) = parse(EX2);
        assert_eq!(1, part2(&rules, &your_ticket, &nearby_tickets));
    }
}
