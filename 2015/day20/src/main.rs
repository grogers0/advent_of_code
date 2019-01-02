use std::cmp::min;
use std::io::{self, Read};

fn first_house_with_enough_presents(houses: &Vec<usize>, min_presents: usize) -> usize {
    for (i, p) in houses.iter().enumerate() {
        if *p >= min_presents {
            return i;
        }
    }
    panic!()
}

// Similar algorithm to sieve of eratosthenes
fn part1(input: &str) -> usize {
    let min_presents = input.trim_end().parse().unwrap();
    let mut houses = vec![0; min_presents/10+1];
    for elf in 1..houses.len() {
        for i in 1..=(houses.len()-1)/elf {
            houses[i*elf] += 10*elf;
        }
    }
    first_house_with_enough_presents(&houses, min_presents)
}

fn part2(input: &str) -> usize {
    let min_presents = input.trim_end().parse().unwrap();
    let mut houses = vec![0; min_presents/10+1];
    for elf in 1..houses.len() {
        for i in 1..=min(50, (houses.len()-1)/elf) {
            houses[i*elf] += 11*elf;
        }
    }
    first_house_with_enough_presents(&houses, min_presents)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("30"), 2);
        assert_eq!(part1("120"), 6);
        assert_eq!(part1("130"), 8);
    }
}
