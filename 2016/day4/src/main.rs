use std::collections::BTreeMap;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

struct Room {
    encrypted_name: String,
    sector: usize,
    checksum: String
}

fn parse(input: &str) -> Vec<Room> {
    lazy_static!{
        static ref RE: Regex = Regex::new("^([a-z-]+)-(\\d+)\\[([a-z]{5})\\]$").unwrap();
    }
    input.lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Room {
                encrypted_name: cap[1].to_string(),
                sector: cap[2].parse().unwrap(),
                checksum: cap[3].to_string()
            }
        })
        .collect()
}

fn real_room(room: &Room) -> bool {
    let mut letter_counts = BTreeMap::new();
    for ch in room.encrypted_name.chars() {
        letter_counts.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    letter_counts.remove(&'-');
    let mut letter_counts = letter_counts.iter().collect::<Vec<_>>();
    letter_counts.sort_by_key(|(ch, count)| (*count, std::u32::MAX - **ch as u32));
    let checksum = letter_counts.iter()
        .rev()
        .map(|(ch, _)| **ch)
        .take(5)
        .collect::<String>();
    checksum == room.checksum
}

fn shift(ch: char, offset: usize) -> char {
    let ch_num = ch as u32 - 'a' as u32;
    let ch_num = (ch_num + offset as u32) % 26;
    (ch_num + 'a' as u32) as u8 as char
}

fn decrypt(room: &Room) -> String {
    room.encrypted_name.chars()
        .map(|ch| if ch == '-' { ' ' } else { shift(ch, room.sector) })
        .collect()
}

fn part1(rooms: &Vec<Room>) -> usize {
    rooms.iter()
        .filter(|room| real_room(*room))
        .map(|room| room.sector)
        .sum()
}

fn part2(rooms: &Vec<Room>) -> usize {
    rooms.iter()
        .filter(|room| real_room(*room))
        .filter(|room| decrypt(room) == "northpole object storage".to_string())
        .next()
        .unwrap().sector
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rooms = parse(&input);

    println!("{}", part1(&rooms));
    println!("{}", part2(&rooms));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn test_part1() {
        assert!(real_room(&parse("aaaaa-bbb-z-y-x-123[abxyz]")[0]));
        assert!(real_room(&parse("a-b-c-d-e-f-g-h-987[abcde]")[0]));
        assert!(real_room(&parse("not-a-real-room-404[oarel]")[0]));
        assert!(!real_room(&parse("totally-real-room-200[decoy]")[0]));
        assert_eq!(part1(&parse(EX)), 1514);
    }

    #[test]
    fn test_part2() {
        assert_eq!(decrypt(&parse("qzmt-zixmtkozy-ivhz-343[qwert]")[0]), "very encrypted name".to_string());
    }

}
