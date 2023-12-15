use std::cmp::min;
use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Range {
    begin: u64,
    len: u64,
}

impl Range {
    fn contains(&self, num: u64) -> bool {
        self.begin <= num &&
            num <= self.begin + self.len
    }
}

struct RangeMapping {
    src_range: Range,
    dst_begin: u64,
}

impl RangeMapping {
    fn map(&self, num: u64) -> u64 {
        debug_assert!(self.src_range.contains(num));
        num - self.src_range.begin + self.dst_begin
    }
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<RangeMapping>>,
}

fn parse(puzzle_input: &str) -> Almanac {
    const SEEDS_STR: &str = "seeds: ";

    let mut paragraphs = puzzle_input.split("\n\n");
    let seeds_input = paragraphs.next().unwrap();
    assert!(seeds_input.starts_with(SEEDS_STR));
    let seeds_input = &seeds_input[SEEDS_STR.len()..];
    let mut seeds = Vec::new();
    for seed_str in seeds_input.split_whitespace() {
        seeds.push(seed_str.parse().unwrap());
    }
    let mut maps = Vec::new();
    while let Some(paragraph) = paragraphs.next() {
        let mut map = Vec::new();
        for line in paragraph.lines().skip(1) {
            let mut parts = line.split_whitespace();
            let dst_begin = parts.next().unwrap().parse().unwrap();
            let src_begin = parts.next().unwrap().parse().unwrap();
            let len = parts.next().unwrap().parse().unwrap();
            let src_range = Range { begin: src_begin, len };
            map.push(RangeMapping { src_range, dst_begin });
        }
        maps.push(map);
    }

    Almanac { seeds, maps }
}

fn lookup_next(map: &[RangeMapping], num: u64) -> u64 {
    for range_mapping in map {
        if range_mapping.src_range.contains(num) {
            return range_mapping.map(num)
        }
    }
    num
}

fn location_for_seed(almanac: &Almanac, seed: u64) -> u64 {
    let mut num = seed;
    for map in &almanac.maps {
        num = lookup_next(&map, num);
    }
    num
}

fn part1(almanac: &Almanac) -> u64 {
    almanac.seeds.iter()
        .map(|&seed| location_for_seed(almanac, seed))
        .min().unwrap()
}

// Returns a split into (overlapping range, non-overlapping ranges)
fn segment_by_overlapping(a: &Range, b: &Range) -> (Option<Range>, Vec<Range>) {
    let mut overlapping = None;
    let mut non_overlapping = Vec::new();
    if a.begin <= b.begin {
        if a.begin < b.begin {
            non_overlapping.push(Range {
                begin: a.begin,
                len: min(b.begin - a.begin, a.len),
            });
        }
        if a.begin + a.len > b.begin {
            overlapping = Some(Range {
                begin: b.begin,
                len: min(a.begin + a.len - b.begin, b.len),
            });
        }
        if a.begin + a.len > b.begin + b.len {
            non_overlapping.push(Range {
                begin: b.begin + b.len,
                len: a.begin + a.len - b.begin - b.len,
            });
        }
    } else {
        if b.begin + b.len > a.begin {
            overlapping = Some(Range {
                begin: a.begin,
                len: min(b.begin + b.len - a.begin, a.len),
            });
            if b.begin + b.len < a.begin + a.len {
                non_overlapping.push(Range {
                    begin: b.begin + b.len,
                    len: a.begin + a.len - b.begin - b.len,
                });
            }
        } else {
            non_overlapping.push(a.clone());
        }
    }
    debug_assert!(overlapping.is_some() || non_overlapping.len() == 1);
    debug_assert_eq!(a.len, overlapping.clone().map(|r| r.len).unwrap_or(0) + non_overlapping.iter().map(|r| r.len).sum::<u64>());
    (overlapping, non_overlapping)
}

fn seed_ranges(almanac: &Almanac) -> Vec<Range> {
    let mut ret = Vec::new();
    let mut seed_iter = almanac.seeds.iter().cloned();
    while let Some(begin) = seed_iter.next() {
        let len = seed_iter.next().unwrap();
        ret.push(Range { begin, len });
    }
    ret
}

fn map_ranges_once(map: &[RangeMapping], mut src_ranges: Vec<Range>) -> Vec<Range> {
    let total_len: u64 = src_ranges.iter().map(|range| range.len).sum();
    let mut dst_ranges = Vec::new();
    'outer: while let Some(range) = src_ranges.pop() {
        for range_mapping in map.iter() {
            let (overlapping, non_overlapping) =
                segment_by_overlapping(&range, &range_mapping.src_range);
            if let Some(overlapping_range) = overlapping {
                for r in non_overlapping {
                    src_ranges.push(r);
                }
                dst_ranges.push(Range {
                    begin: overlapping_range.begin -
                        range_mapping.src_range.begin +
                        range_mapping.dst_begin,
                    len: overlapping_range.len,
                });
                continue 'outer;
            }
        }
        dst_ranges.push(range.clone());
    }
    debug_assert_eq!(total_len, dst_ranges.iter().map(|range| range.len).sum());
    dst_ranges
}

fn map_ranges_all(maps: &[Vec<RangeMapping>], mut ranges: Vec<Range>) -> Vec<Range> {
    for map in maps.iter() {
        ranges = map_ranges_once(&map, ranges);
    }
    ranges
}

fn part2(almanac: &Almanac) -> u64 {
    let mut location_ranges =
        map_ranges_all(&almanac.maps, seed_ranges(almanac));
    location_ranges.iter().map(|range| range.begin).min().unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let almanac = parse(&puzzle_input);
    println!("{}", part1(&almanac));
    println!("{}", part2(&almanac));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 46);
    }
}
