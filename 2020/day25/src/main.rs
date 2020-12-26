use std::io::{self, Read};

fn parse(puzzle_input: &str) -> [u64; 2] {
    let mut lines = puzzle_input.lines();
    let card_pub_key = lines.next().unwrap().parse().unwrap();
    let door_pub_key = lines.next().unwrap().parse().unwrap();
    assert!(lines.next().is_none());
    [card_pub_key, door_pub_key]
}

fn transform(subject_num: u64, loop_size: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val = (val * subject_num) % 20201227;
    }
    val
}

fn invert_loop_size(pub_key: u64) -> u64 {
    let mut val = 1;
    for iters in 0.. {
        if val == pub_key { return iters }
        val = (val * 7) % 20201227;
    }
    unreachable!()
}

fn part1(card_pub_key: u64, door_pub_key: u64) -> u64 {
    let card_loop_size = invert_loop_size(card_pub_key);
    let door_loop_size = invert_loop_size(door_pub_key);
    let encr_key = transform(door_pub_key, card_loop_size);
    debug_assert_eq!(encr_key, transform(card_pub_key, door_loop_size));
    encr_key
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let [card_pub_key, door_pub_key] = parse(&puzzle_input);

    println!("{}", part1(card_pub_key, door_pub_key));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(14897079, part1(5764801, 17807724));
    }
}
