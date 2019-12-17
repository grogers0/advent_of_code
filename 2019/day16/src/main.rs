use std::io::{self, Read};

const PAT: [i32; 4] = [0, 1, 0, -1];

fn parse(signal: &str) -> Vec<u8> {
    signal.trim().chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect()
}

fn fft_slow(signal: Vec<u8>, offset: usize) -> Vec<u8> {
    (0 .. signal.len()).into_iter().map(|i| {
        let mut val = 0;
        for j in i .. signal.len() {
            val += signal[j] as i32 * PAT[(((j + offset + 1) / (i + offset + 1)) % 4)];
        }
        (val.abs() % 10) as u8
    }).collect()
}

// When the offset is more than half the total length, the pattern ends up as an all ones
// upper-triangular matrix, like:
//
// 1 1 1 1 
// 0 1 1 1
// 0 0 1 1
// 0 0 0 1
//
// So we can just sum the elements in reverse order
fn fft_fast(signal: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::with_capacity(signal.len());

    let mut sum = 0;
    for val in signal.iter().rev() {
        sum += *val as i32;
        output.push((sum.abs() % 10) as u8);
    }
    output.reverse();
    output
}

fn fft(signal: Vec<u8>, offset: usize) -> Vec<u8> {
    if signal.len() >= offset {
        fft_slow(signal, offset)
    } else {
        fft_fast(signal)
    }
}

fn signal_offset(signal: &Vec<u8>) -> usize {
    let signal = &signal[0..7];
    let mut offset = 0;
    for val in signal {
        offset = offset * 10 + (*val as usize);
    }
    offset
}

fn iter_fft(mut signal: Vec<u8>, iterations: usize, offset: usize) -> Vec<u8> {
    for _ in 0 .. iterations {
        signal = fft(signal, offset);
    }
    signal
}

fn part1(input: &str) -> String {
    iter_fft(parse(input), 100, 0)[0..8].iter().map(|i| format!("{}", i)).collect()
}

fn part2(input: &str) -> String {
    let signal_once = parse(input);
    let offset = signal_offset(&signal_once);
    let mut signal = Vec::with_capacity(signal_once.len() * 10_000);
    for _ in 0..10_000 {
        for val in signal_once.iter() {
            signal.push(*val);
        }
    }
    // Only digits at or to the right of a particular digit can affect it's value
    let signal = signal.split_off(offset);
    iter_fft(signal, 100, offset)[0..8].iter().map(|i| format!("{}", i)).collect()
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
        assert_eq!(iter_fft(parse("12345678"), 1, 0), parse("48226158"));
        assert_eq!(iter_fft(parse("12345678"), 2, 0), parse("34040438"));
        assert_eq!(iter_fft(parse("12345678"), 3, 0), parse("03415518"));
        assert_eq!(iter_fft(parse("12345678"), 4, 0), parse("01029498"));

        assert_eq!(part1("80871224585914546619083218645595"), "24176176");
        assert_eq!(part1("19617804207202209144916044189917"), "73745418");
        assert_eq!(part1("69317163492948606335995924319873"), "52432133");
    }

    #[test]
    fn test_part2() {
        assert_eq!(iter_fft(parse("2345678"), 1, 1), parse("8226158"));
        assert_eq!(iter_fft(parse("45678"), 1, 3), parse("26158"));
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2("03036732577212944063491565474664"), "84462026");
    }

    #[test]
    fn test_part2_ex2() {
        assert_eq!(part2("02935109699940807407585447034323"), "78725270");
    }

    #[test]
    fn test_part2_ex3() {
        assert_eq!(part2("03081770884921959731165446850517"), "53553731");
    }
}
