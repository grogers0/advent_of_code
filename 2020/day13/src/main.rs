use std::io::{self, Read};

type Bus = u16;

fn parse(puzzle_input: &str) -> (u64, Vec<(Bus, usize)>) {
    let mut lines = puzzle_input.lines();
    let earliest_departure = lines.next().unwrap().parse().unwrap();
    let buses = lines.next().unwrap().split(',')
        .enumerate()
        .filter(|&(_, bus_str)| bus_str != "x")
        .map(|(i, bus_str)| (bus_str.parse().unwrap(), i))
        .collect();
    assert!(lines.next().is_none());
    (earliest_departure, buses)
}

fn wait_time(t: u64, bus: Bus) -> Bus {
    (bus - (t % bus as u64) as Bus) % bus
}

fn part1(earliest_departure: u64, buses: &Vec<(Bus, usize)>) -> u64 {
    let mut best_bus = 0;
    let mut best_wait = Bus::MAX;
    for &(bus, _) in buses {
        let wait = wait_time(earliest_departure, bus);
        if wait < best_wait {
            best_wait = wait;
            best_bus = bus;
        }
    }
    best_bus as u64 * best_wait as u64
}

fn part2(buses: &Vec<(Bus, usize)>) -> u64 {
    let mut t = 0;
    let mut mult = 1;
    // NOTE - Luckily the bus IDs given are coprime, I'm pretty sure otherwise this wouldn't find the first occurence
    for &(bus, offset) in buses {
        while wait_time(t + offset as u64, bus) != 0 {
            t += mult;
        }
        mult *= bus as u64;
    }
    assert!(buses.iter().all(|&(bus, offset)| wait_time(t + offset as u64, bus) == 0));
    t
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let (earliest_departure, buses) = parse(&puzzle_input);

    println!("{}", part1(earliest_departure, &buses));
    println!("{}", part2(&buses));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() {
        let (earliest_departure, buses) = parse(EX);
        assert_eq!(295, part1(earliest_departure, &buses));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1068781, part2(&parse(EX).1));

        assert_eq!(3417, part2(&parse("0\n17,x,13,19").1));
        assert_eq!(754018, part2(&parse("0\n67,7,59,61").1));
        assert_eq!(779210, part2(&parse("0\n67,x,7,59,61").1));
        assert_eq!(1261476, part2(&parse("0\n67,7,x,59,61").1));
        assert_eq!(1202161486, part2(&parse("0\n1789,37,47,1889").1));
    }
}
