use std::io::{self, Read};

use bit_vec::BitVec;

// true if there is a trap
fn parse_row(input: &str) -> BitVec {
    input.chars().map(|ch| {
        match ch {
            '.' => false,
            '^' => true,
            _ => panic!()
        }
    }).collect()
}

#[allow(dead_code)]
fn map_to_string(map: &Vec<BitVec>) -> String {
    let mut out = String::new();
    let mut first = true;
    for row in map.iter() {
        if first {
            first = false;
        } else {
            out.push('\n');
        }
        for trap in row.iter() {
            out.push(if trap { '^' } else { '.' });
        }
    }
    out
}

// Each tile is true if there is a trap there
fn generate_map(input: &str, rows: usize) -> Vec<BitVec> {
    let first_row = parse_row(input.trim_end());
    let width = first_row.len();
    let mut map = vec![first_row];
    for y in 0..rows-1 {
        let row = (0..width).map(|x| {
            let left = if x == 0 { false } else { map[y][x - 1] };
            let center = map[y][x];
            let right = if x == width - 1 { false } else { map[y][x + 1] };
            (left && center && !right) ||
                (!left && center && right) ||
                (left && !center && !right) ||
                (!left && !center && right)
        }).collect();
        map.push(row);
    }
    map
}

fn count_safe_tiles(map: &Vec<BitVec>) -> usize {
    map.iter().map(|row| row.iter().filter(|trap| !*trap).count()).sum()
}

fn part1(input: &str) -> usize {
    count_safe_tiles(&generate_map(input, 40))
}

// NOTE - this is fast enough, but since we don't need to generate the full map it would be better
// instead to count the tiles as we go and only keep the current row
fn part2(input: &str) -> usize {
    count_safe_tiles(&generate_map(input, 400000))
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
        assert_eq!(count_safe_tiles(&generate_map(".^^.^.^^^^", 10)), 38);
    }
}
