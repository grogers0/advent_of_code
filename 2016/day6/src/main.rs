use std::collections::BTreeMap;
use std::io::{self, Read};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn char_counts(mut chars: impl Iterator<Item=char>) -> BTreeMap<char, usize> {
    let mut counts = BTreeMap::new();
    while let Some(ch) = chars.next() {
        counts.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    counts    
}

fn most_frequent(chars: impl Iterator<Item=char>) -> char {
    char_counts(chars).into_iter().max_by_key(|(_,cnt)| *cnt).unwrap().0
}

fn least_frequent(chars: impl Iterator<Item=char>) -> char {
    char_counts(chars).into_iter().min_by_key(|(_,cnt)| *cnt).unwrap().0
}


fn part1(input: &str) -> String {
    let signal = parse(input);
    (0..signal[0].len()).map(|i| most_frequent(signal.iter().map(|chars| chars[i]))).collect()
}

fn part2(input: &str) -> String {
    let signal = parse(input);
    (0..signal[0].len()).map(|i| least_frequent(signal.iter().map(|chars| chars[i]))).collect()
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

    const EX: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), "easter".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), "advent".to_string());
    }
}
