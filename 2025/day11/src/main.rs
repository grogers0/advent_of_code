use std::io::{self, Read};
use std::collections::HashMap;

fn parse(puzzle_input: &str) -> HashMap<String, Vec<String>> {
    let mut ret = HashMap::new();
    for line in puzzle_input.lines() {
        let mut sp = line.split(": ");
        let device = sp.next().unwrap().to_string();
        let outputs = sp.next().unwrap().split(" ")
            .map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(sp.next().is_none());

        let inserted = ret.insert(device, outputs);
        assert!(inserted.is_none());
    }
    ret
}

fn part1(devices: &HashMap<String, Vec<String>>) -> usize {
    fn count_paths(devices: &HashMap<String, Vec<String>>, curr: &str)
        -> usize {
        if curr == "out" { return 1; }
        let mut cnt = 0;
        for next in &devices[curr] {
            cnt += count_paths(devices, next);
        }
        cnt
    }

    count_paths(devices, "you")
}

fn part2(devices: &HashMap<String, Vec<String>>) -> usize {
    fn count_paths(devices: &HashMap<String, Vec<String>>, curr: &str,
        dac: bool, fft: bool,
        memo: &mut HashMap<(String, bool, bool), usize>) -> usize {
        if curr == "out" {
            return if dac && fft { 1 } else { 0 };
        } else if let Some(cached) = memo.get(&(curr.to_string(), dac, fft)) {
            return *cached;
        }

        let mut cnt = 0;
        for next in &devices[curr] {
            let dac = dac || next == "dac";
            let fft = fft || next == "fft";
            cnt += count_paths(devices, next, dac, fft, memo);
        }
        memo.insert((curr.to_string(), dac, fft), cnt);
        cnt
    }

    let mut memo = HashMap::new();
    count_paths(devices, "svr", false, false, &mut memo)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let devices = parse(&puzzle_input);
    println!("{}", part1(&devices));
    println!("{}", part2(&devices));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX1)), 5);
    }

    const EX2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX2)), 2);
    }
}
