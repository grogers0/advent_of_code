use std::io::{self, Read};
use std::collections::{HashSet, HashMap};

fn parse(puzzle_input: &str) -> Vec<u64> {
    puzzle_input.lines().map(|line| line.parse::<u64>().unwrap()).collect()
}

fn mix(secret: u64, val: u64) -> u64 {
    secret ^ val
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next(mut secret: u64) -> u64 {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
    secret
}

fn iterate(mut secret: u64, rounds: usize) -> u64 {
    for _ in 0..rounds {
        secret = next(secret);
    }
    secret
}

fn part1(secrets: &[u64]) -> u64 {
    let mut sum = 0;
    for &secret in secrets {
        sum += iterate(secret, 2000);
    }
    sum
}

fn price_of(secret: u64) -> u8 {
    (secret % 10) as u8
}

fn price_delta(p1: u8, p2: u8) -> i8 {
    p1 as i8 - p2 as i8
}

fn part2(secrets: &[u64]) -> u64 {
    let mut sell_prices = HashMap::new();
    for secret in secrets {
        let mut curr = [0i8; 4];
        let mut secret = *secret;
        let mut old_price = price_of(secret);
        let mut seen = HashSet::new();
        for round in 0..2000 {
            secret = next(secret);
            let price = price_of(secret);
            curr = [price_delta(price, old_price), curr[0], curr[1], curr[2]];
            old_price = price;
            if round >= 3 {
                if !seen.insert(curr) { continue; }
                sell_prices.entry(curr).and_modify(|p| *p += price as u64).or_insert(price as u64);
            }
        }
    }

    let mut best = 0;
    for (_, price) in sell_prices {
        if price > best { best = price; }
    }
    best
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let secrets = parse(&puzzle_input);
    println!("{}", part1(&secrets));
    println!("{}", part2(&secrets));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "1
10
100
2024";

    const EX2: &str = "1
2
3
2024";

    #[test]
    fn test_part1() {
        assert_eq!(iterate(123, 1), 15887950);
        assert_eq!(iterate(123, 2), 16495136);
        assert_eq!(iterate(123, 3), 527345);
        assert_eq!(iterate(123, 4), 704524);
        assert_eq!(iterate(123, 5), 1553684);
        assert_eq!(iterate(123, 6), 12683156);
        assert_eq!(iterate(123, 7), 11100544);
        assert_eq!(iterate(123, 8), 12249484);
        assert_eq!(iterate(123, 9), 7753432);
        assert_eq!(iterate(123, 10), 5908254);

        assert_eq!(part1(&parse(EX1)), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX2)), 23);
    }
}
