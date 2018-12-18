use std::io::{self, Read};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Mode {
    Normal, MarkerLen, MarkerRepeat, Subsequent
}

// NOTE - we don't want to build the full string because it takes many gigabytes
fn decompressed_length(input: &str, decompress_subsequent: bool) -> usize {
    let mut len = 0;

    let mut mode = Mode::Normal;
    let mut marker_len = 0;
    let mut marker_repeat = 0;
    let mut subsequent = String::new();
    for ch in input.chars() {
        if ch.is_whitespace() { continue; }
        match mode {
            Mode::Normal => {
                if ch == '(' {
                    mode = Mode::MarkerLen;
                    marker_len = 0;
                } else {
                    len += 1;
                }
            },
            Mode::MarkerLen => {
                if ch == 'x' {
                    mode = Mode::MarkerRepeat;
                    marker_repeat = 0;
                } else {
                    marker_len = marker_len * 10 + ch.to_digit(10).unwrap() as usize;
                }
            },
            Mode::MarkerRepeat => {
                if ch == ')' {
                    mode = Mode::Subsequent;
                    subsequent.clear();
                } else {
                    marker_repeat = marker_repeat * 10 + ch.to_digit(10).unwrap() as usize;
                }
            },
            Mode::Subsequent => {
                assert!(marker_len > 0);
                if marker_len == 1 {
                    subsequent.push(ch);
                    mode = Mode::Normal;
                    let subsequent_len = if decompress_subsequent {
                        decompressed_length(&subsequent, true)
                    } else {
                        subsequent.chars().count()
                    };
                    len += subsequent_len * marker_repeat;
                } else {
                    marker_len -= 1;
                    subsequent.push(ch);
                }
            }
        }
    }
    assert_eq!(mode, Mode::Normal);
    len
}

fn part1(input: &str) -> usize {
    decompressed_length(input, false)
}

fn part2(input: &str) -> usize {
    decompressed_length(input, true)
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
        assert_eq!(part1("ADVENT"), "ADVENT".len());
        assert_eq!(part1("A(1x5)BC"), "ABBBBBC".len());
        assert_eq!(part1("(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(part1("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG".len());
        assert_eq!(part1("(6x1)(1x3)A"), "(1x3)A".len());
        assert_eq!(part1("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY".len());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(part2("X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY".len());
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
    }
}
