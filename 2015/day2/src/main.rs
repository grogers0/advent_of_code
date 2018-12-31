use std::io::{self, Read};

use regex::Regex;

fn parse(input: &str) -> Vec<[u64; 3]> {
    let re = Regex::new("^(\\d+)x(\\d+)x(\\d+)$").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            [cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()]
        })
        .collect()
}

fn surface_area([l,w,h]: [u64; 3]) -> u64 {
    2*l*w + 2*w*h + 2*h*l
}

fn smallest_face_area([l,w,h]: [u64; 3]) -> u64 {
    let mut areas = [l*w, w*h, h*l];
    areas.sort();
    areas[0]
}

fn volume([l,w,h]: [u64; 3]) -> u64 {
    l*w*h
}

fn smallest_face_perimeter([l,w,h]: [u64; 3]) -> u64 {
    let mut perims = [2*(l+w), 2*(w+h), 2*(h+l)];
    perims.sort();
    perims[0]
}

fn part1(input: &str) -> u64 {
    let boxes = parse(input);
    boxes.into_iter()
        .map(|dims| surface_area(dims) + smallest_face_area(dims))
        .sum()
}

fn part2(input: &str) -> u64 {
    let boxes = parse(input);
    boxes.into_iter()
        .map(|dims| smallest_face_perimeter(dims) + volume(dims))
        .sum()
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
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }
}
