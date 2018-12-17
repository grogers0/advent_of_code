use std::collections::VecDeque;
use std::io::{self, Read};

pub fn reverse_circular<T: Copy>(list: &mut Vec<T>, mut start: usize, mut len: usize) {
    while len > 1 {
        let tmp = list[start % 256];
        list[start % 256] = list[(start + len - 1) % 256];
        list[(start + len - 1) % 256] = tmp;
        start += 1;
        len -= 2;
    }
}

fn parse_lengths_as_ascii(input: &str) -> Vec<usize> {
    let mut lengths: Vec<usize> = input.trim_end().chars()
        .map(|ch| ch as usize)
        .collect();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);
    lengths
}

// TODO - how to reuse code across days?
fn knot_hash(input: &str) -> Vec<u8> {
    let lengths = parse_lengths_as_ascii(input);
    let mut list: Vec<u8> = Vec::new();
    for i in 0 ..= 255 { list.push(i) }

    let mut curr = 0;
    let mut skip = 0;
    for _round in 0..64 {
        for length in &lengths {
            reverse_circular(&mut list, curr, *length);
            curr += *length + skip;
            skip += 1;
        }
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc,x| acc ^ x))
        .collect()
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for i in 0 ..= 127 {
        let hash_val = knot_hash(&format!("{}-{}", input, i));
        for byte in hash_val {
            sum += byte.count_ones();
        }
    }
    sum
}

fn idx(x: usize, y: usize) -> usize {
    y * 128 + x
}

fn mask_region(matrix: &mut Vec<bool>, start_x: usize, start_y: usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    while let Some((x, y)) = queue.pop_front() {
        if !matrix[idx(x, y)] { continue }
        matrix[idx(x, y)] = false;

        if x > 0 { queue.push_back((x-1, y)) }
        if x < 127 { queue.push_back((x+1, y)) }
        if y > 0 { queue.push_back((x, y-1)) }
        if y < 127 { queue.push_back((x, y+1)) }
    }
}

fn part2(input: &str) -> usize {
    let mut matrix = vec![false; 128*128];
    for y in 0 ..= 127 {
        let hash_val = knot_hash(&format!("{}-{}", input, y));
        for (x_8, byte) in hash_val.iter().enumerate() {
            let mut byte = *byte;
            for i in 0..8 {
                let bit = (byte & 0x80) != 0;
                byte <<= 1;
                matrix[idx(x_8 * 8 + i, y)] = bit;
            }
        }
    }

    let mut num_regions = 0;
    for y in 0..=127 {
        for x in 0..=127 {
            if matrix[idx(x, y)] {
                mask_region(&mut matrix, x, y);
                num_regions += 1;
            }
        }
    }

    num_regions
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
        assert_eq!(part1("flqrgnkx"), 8108);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("flqrgnkx"), 1242);
    }

}
