use std::io::{self, Read};

fn parse_available(avail_input: &str) -> Vec<String> {
    avail_input.split(", ").map(|s| s.to_string()).collect()
}

fn parse_designs(designs_input: &str) -> Vec<String> {
    designs_input.lines().map(|s| s.to_string()).collect()
}

fn parse(puzzle_input: &str) -> (Vec<String>, Vec<String>) {
    let mut sp = puzzle_input.split("\n\n");
    let available = parse_available(sp.next().unwrap());
    let designs = parse_designs(sp.next().unwrap());
    assert!(sp.next().is_none());
    (available, designs)
}

fn num_possible_designs(available: &[String], design: &str) -> usize {
    let mut counts = vec![0; design.len() + 1];
    counts[0] = 1;
    for idx in 0..design.len() {
        let cnt = counts[idx];
        for towel in available {
            if design[idx..].starts_with(towel) {
                counts[idx + towel.len()] += cnt;
            }
        }
    }
    counts[design.len()]
}

fn part1(available: &[String], designs: &[String]) -> usize {
    designs.iter().filter(|design| num_possible_designs(available, design) != 0).count()
}

fn part2(available: &[String], designs: &[String]) -> usize {
    designs.iter().map(|design| num_possible_designs(available, design)).sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (available, designs) = parse(&puzzle_input);
    println!("{}", part1(&available, &designs));
    println!("{}", part2(&available, &designs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        let (available, designs) = parse(EX);
        assert_eq!(part1(&available, &designs), 6);
    }

    #[test]
    fn test_part2() {
        let (available, designs) = parse(EX);
        assert_eq!(part2(&available, &designs), 16);
    }
}
