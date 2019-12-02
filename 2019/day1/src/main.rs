use std::io::{self, Read};

fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn fuel_needed(module_mass: i32) -> i32 {
    let fuel = (module_mass / 3) - 2;
    if fuel >= 0 { fuel } else { 0 }
}

fn fuel_needed_including_fuel(module_mass: i32) -> i32 {
    let mut fuel = fuel_needed(module_mass);
    let mut tot_fuel = fuel;
    while fuel > 0 {
        fuel = fuel_needed(fuel);
        tot_fuel += fuel;
    }
    tot_fuel
}

fn part1(input: &str) -> i32 {
    parse_input(input).into_iter().map(|mass| fuel_needed(mass)).sum()
}

fn part2(input: &str) -> i32 {
    parse_input(input).into_iter().map(|mass| fuel_needed_including_fuel(mass)).sum()
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
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 654);
        assert_eq!(fuel_needed(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(fuel_needed_including_fuel(14), 2);
        assert_eq!(fuel_needed_including_fuel(1969), 966);
        assert_eq!(fuel_needed_including_fuel(100756), 50346);
    }
}
