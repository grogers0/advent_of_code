use std::collections::HashMap;
use std::io::{self, Read};

enum Cmd {
    Mask([Option<bool>; 36]),
    Write(u64, u64)
}

impl Cmd {
    fn parse(s: &str) -> Cmd {
        let mut parts = s.split(" = ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        assert!(parts.next().is_none());
        if first == "mask" {
            assert!(second.chars().count() == 36);
            let mut mask = [None; 36];
            for (i, ch) in second.chars().enumerate() {
                let bit = match ch {
                    'X' => None,
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => panic!()
                };
                mask[36 - i - 1] = bit;
            }
            Cmd::Mask(mask)
        } else {
            assert!(first.starts_with("mem[") && first.ends_with("]"));
            let addr = first[4..(first.len()-1)].parse().unwrap();
            let val = second.parse().unwrap(); 
            assert!(addr < (1 << 36) && val < (1 << 36));
            Cmd::Write(addr, val)
        }
    }
}

fn parse(puzzle_input: &str) -> Vec<Cmd> {
    puzzle_input.lines().map(|line| Cmd::parse(line)).collect()
}

fn part1(cmds: &Vec<Cmd>) -> u64 {
    let mut mem = HashMap::new();
    let mut curmask = [Some(true); 36];
    for cmd in cmds {
        match cmd {
            Cmd::Mask(mask) => {
                curmask = mask.clone();
            },
            Cmd::Write(addr, mut val) => {
                for i in 0..36 {
                    match curmask[i] {
                        Some(false) => val &= u64::MAX - (1 << i),
                        Some(true) => val |= 1 << i,
                        None => ()
                    }
                }
                mem.insert(addr, val);
            }
        }
    }
    mem.values().sum()
}

fn part2(cmds: &Vec<Cmd>) -> u64 {
    fn write_addrs(mem: &mut HashMap<u64, u64>, addr: u64, val: u64, mask: &[Option<bool>; 36], bitidx: u64) {
        if bitidx == 36 {
            mem.insert(addr, val);
            return;
        }
        match mask[bitidx as usize] {
            Some(false) => write_addrs(mem, addr, val, mask, bitidx + 1),
            Some(true) => write_addrs(mem, addr | (1 << bitidx), val, mask, bitidx + 1),
            None => {
                write_addrs(mem, addr & (u64::MAX - (1 << bitidx)), val, mask, bitidx + 1);
                write_addrs(mem, addr | (1 << bitidx), val, mask, bitidx + 1);
            }
        }
    }

    let mut mem = HashMap::new();
    let mut curmask = [Some(true); 36];
    for cmd in cmds {
        match cmd {
            Cmd::Mask(mask) => {
                curmask = mask.clone();
            },
            Cmd::Write(addr, val) => {
                write_addrs(&mut mem, *addr, *val, &curmask, 0);
            }
        }
    }
    mem.values().sum()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let cmds = parse(&puzzle_input);

    println!("{}", part1(&cmds));
    println!("{}", part2(&cmds));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const EX2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part1() {
        assert_eq!(165, part1(&parse(EX1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(208, part2(&parse(EX2)));
    }
}
