use std::io::{self, Read};

enum Sequence {
    Supernet(String),
    Hypernet(String)
}

fn parse_addr(input: &str) -> Vec<Sequence> {
    let mut ret = Vec::new();
    let mut seq = String::new();
    let mut in_brackets = false;
    for ch in input.chars() {
        if ch == '[' {
            assert!(!in_brackets);
            in_brackets = true;
            ret.push(Sequence::Supernet(seq));
            seq = String::new();
        } else if ch == ']' {
            assert!(in_brackets);
            in_brackets = false;
            ret.push(Sequence::Hypernet(seq));
            seq = String::new();
        } else {
            seq.push(ch);
        }
    }
    ret.push(Sequence::Supernet(seq));
    ret
}

fn has_abba(sequence: &str) -> bool {
    let sequence = sequence.chars().collect::<Vec<_>>();
    sequence.windows(4).any(|wnd| match wnd {
        &[a, b, c, d] if a != b && a == d && b == c => true,
        _ => false
    })
}

fn supernets(ip: &Vec<Sequence>) -> impl Iterator<Item=&str> {
    ip.iter().map(|seq| match seq {
        Sequence::Supernet(s) => Some(s),
        Sequence::Hypernet(_) => None
    }).flat_map(|seq_opt| seq_opt).map(|seq| seq.as_str())
}

fn hypernets(ip: &Vec<Sequence>) -> impl Iterator<Item=&str> {
    ip.iter().map(|seq| match seq {
        Sequence::Supernet(_) => None,
        Sequence::Hypernet(s) => Some(s)
    }).flat_map(|seq_opt| seq_opt).map(|seq| seq.as_str())
}

fn supports_tls(ip: &Vec<Sequence>) -> bool {
    supernets(ip).any(|seq| has_abba(seq)) &&
        !hypernets(ip).any(|seq| has_abba(seq))
}

fn part1(input: &str) -> usize {
    input.lines()
        .filter(|line| supports_tls(&parse_addr(line)))
        .count()
}

// Returns vec of (a, b)
fn find_aba<'a>(sequences: impl Iterator<Item=&'a str>) -> Vec<(char, char)> {
    let mut ret = Vec::new();
    for sequence in sequences {
        let sequence = sequence.chars().collect::<Vec<_>>();
        for window in sequence.windows(3) {
            if let &[a, b, c] = window {
                if a != b && a == c { ret.push((a, b)); }
            }
        }
    }
    ret
}

fn supports_ssl(ip: &Vec<Sequence>) -> bool {
    for (a, b) in find_aba(supernets(ip)) {
        if find_aba(hypernets(ip)).contains(&(b, a)) {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> usize {
    input.lines()
        .filter(|line| supports_ssl(&parse_addr(line)))
        .count()
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
        assert!(supports_tls(&parse_addr("abba[mnop]qrst")));
        assert!(!supports_tls(&parse_addr("abcd[bddb]xyyx")));
        assert!(!supports_tls(&parse_addr("aaaa[qwer]tyui")));
        assert!(supports_tls(&parse_addr("ioxxoj[asdfgh]zxcvbn")));
    }

    #[test]
    fn test_part2() {
        assert!(supports_ssl(&parse_addr("aba[bab]xyz")));
        assert!(!supports_ssl(&parse_addr("xyx[xyx]xyx")));
        assert!(supports_ssl(&parse_addr("aaa[kek]eke")));
        assert!(supports_ssl(&parse_addr("zazbz[bzb]cdb")));
    }
}
