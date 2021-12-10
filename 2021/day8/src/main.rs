use std::collections::{BTreeSet, HashMap};
use std::io::{self, Read};
use once_cell::sync::OnceCell;

type UniquePatterns = [String; 10];
type OutputPatterns = [String; 4];

fn parse(puzzle_input: &str) -> Vec<(UniquePatterns, OutputPatterns)> {
    puzzle_input.lines().map(|line| {
        let mut pipe_iter = line.split(" | ");
        const EMPTY: String = String::new();
        let mut unique_patterns: UniquePatterns = [EMPTY; 10];
        let mut output_patterns: OutputPatterns = [EMPTY; 4];

        let mut max_pattern = 0;
        for (i, s) in pipe_iter.next().unwrap().split(" ").enumerate() {
            unique_patterns[i] = s.to_string();
            max_pattern = i;
        }
        assert_eq!(max_pattern, 9);
        for (i, s) in pipe_iter.next().unwrap().split(" ").enumerate() {
            output_patterns[i] = s.to_string();
            max_pattern = i;
        }
        assert_eq!(max_pattern, 3);
        assert!(pipe_iter.next().is_none());

        (unique_patterns, output_patterns)
    }).collect()
}

fn part1(signals: &[(UniquePatterns, OutputPatterns)]) -> u32 {
    let mut cnt = 0;
    for (_, output_patterns) in signals {
        for pat in output_patterns {
            cnt += match pat.len() {
                2 => 1, // digit: 1
                3 => 1, // digit: 7
                4 => 1, // digit: 4
                7 => 1, // digit: 8
                _ => 0 // unknown digit
            };
        }
    }
    cnt
}

fn str_to_charset(s: &str) -> BTreeSet<char> {
    let mut charset = BTreeSet::new();
    for ch in s.chars() {
        charset.insert(ch);
    }
    charset
}

fn map_pattern_to_digit(pattern: &str, wire_mapping: &HashMap<char, char>) -> Option<u32> {
    static DISPLAY_MAPPING_CELL: OnceCell<HashMap<BTreeSet<char>, u32>> = OnceCell::new();
    let display_mapping = DISPLAY_MAPPING_CELL.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(str_to_charset("abcefg"), 0);
        map.insert(str_to_charset("cf"), 1);
        map.insert(str_to_charset("acdeg"), 2);
        map.insert(str_to_charset("acdfg"), 3);
        map.insert(str_to_charset("bcdf"), 4);
        map.insert(str_to_charset("abdfg"), 5);
        map.insert(str_to_charset("abdefg"), 6);
        map.insert(str_to_charset("acf"), 7);
        map.insert(str_to_charset("abcdefg"), 8);
        map.insert(str_to_charset("abcdfg"), 9);
        map
    });

    let mapped_pattern: BTreeSet<char> = pattern.chars()
        .map(|wire| wire_mapping[&wire]).collect();
    display_mapping.get(&mapped_pattern).cloned()
}

// Determine the mapping of input signal wire -> output signal wire
fn decode_wire_mapping(unique_patterns: &UniquePatterns) -> HashMap<char, char> {
    // This is plenty fast when compiled with optimizations, but obviously pretty naive
    fn check_each_mapping(unique_patterns: &UniquePatterns,
        wire_mapping: &mut HashMap<char, char>,
        remaining_wires: &mut BTreeSet<char>) -> bool
    {
        if remaining_wires.is_empty() {
            return unique_patterns.iter().all(|pattern| {
                map_pattern_to_digit(pattern, wire_mapping).is_some()
            });
        } else {
            let wires: Vec<char> = remaining_wires.iter().cloned().collect();
            for wire in wires {
                remaining_wires.remove(&wire);
                wire_mapping.insert(wire, ('a' as u8 + remaining_wires.len() as u8) as char);
                if check_each_mapping(unique_patterns, wire_mapping, remaining_wires) {
                    return true
                }
                wire_mapping.remove(&wire);
                remaining_wires.insert(wire);
            }
            return false
        }
    }
    let mut wire_mapping = HashMap::new();
    let mut remaining_wires = str_to_charset("abcdefg");
    assert!(check_each_mapping(unique_patterns, &mut wire_mapping, &mut remaining_wires));
    wire_mapping
}

fn calc_output(wire_mapping: &HashMap<char, char>, output_patterns: &OutputPatterns) -> u32 {
    let mut ret = 0;
    for output_pattern in output_patterns {
        ret = ret * 10 + map_pattern_to_digit(output_pattern, wire_mapping).unwrap();
    }
    ret
}


fn part2(signals: &[(UniquePatterns, OutputPatterns)]) -> u32 {
    let mut sum = 0;
    for (unique_patterns, output_patterns) in signals {
        let wire_mapping = decode_wire_mapping(unique_patterns);
        sum += calc_output(&wire_mapping, output_patterns);
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let signals = parse(&puzzle_input);
    println!("{}", part1(&signals));
    println!("{}", part2(&signals));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";


    #[test]
    fn test_part1() {
        assert_eq!(26, part1(&parse(EX)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(61229, part2(&parse(EX)));
    }
}
