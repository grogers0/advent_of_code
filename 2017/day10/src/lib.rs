use crate::reverse_circular::reverse_circular;

mod reverse_circular;

fn parse_lengths(input: &str) -> Vec<usize> {
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

pub fn knot_hash(input: &str) -> Vec<u8> {
    let lengths = parse_lengths(input);
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

