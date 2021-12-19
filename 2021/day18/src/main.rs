use std::fmt;
use std::io::{self, Read};
use std::iter::Peekable;
use std::ops::Add;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SnailfishElem {
    Regular(u8),
    Pair(Box<Snailfish>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snailfish([SnailfishElem; 2]);

impl Snailfish {
    pub fn parse(s: &str) -> Snailfish {
        let mut chars = s.chars().peekable();
        Self::parse_(&mut chars)
    }

    fn parse_<'a>(chars: &mut Peekable<Chars<'a>>) -> Snailfish {
        assert_eq!('[', chars.next().unwrap());
        let a = Self::parse_elem(chars);
        assert_eq!(',', chars.next().unwrap());
        let b = Self::parse_elem(chars);
        assert_eq!(']', chars.next().unwrap());
        Snailfish([a, b])
    }

    fn parse_elem<'a>(chars: &mut Peekable<Chars<'a>>) -> SnailfishElem {
        if *chars.peek().unwrap() == '[' {
            SnailfishElem::Pair(Box::new(Self::parse_(chars)))
        } else {
            let num = chars.next().unwrap().to_digit(10).unwrap();
            SnailfishElem::Regular(num as u8)
        }
    }

    fn reduce(&mut self) {
        while self.try_explode() || self.try_split() { }
    }

    fn try_explode(&mut self) -> bool {
        self.try_explode_recur(0).0
    }

    fn try_explode_recur(&mut self, depth: usize) -> (bool, [u8; 2]) {
        let (exploded, [left, right]) = self.0[0].try_explode_recur(depth + 1);
        if exploded {
            self.0[1].add_to_leftmost(right);
            return (true, [left, 0]);
        }
        let (exploded, [left, right]) = self.0[1].try_explode_recur(depth + 1);
        if exploded {
            self.0[0].add_to_rightmost(left);
            return (true, [0, right]);
        }
        (false, [0, 0])
    }

    fn try_split(&mut self) -> bool {
        self.0[0].try_split() || self.0[1].try_split()
    }

    fn magnitude(&self) -> u32 {
        3 * self.0[0].magnitude() + 2 * self.0[1].magnitude()
    }
}

impl SnailfishElem {
    fn try_explode_recur(&mut self, depth: usize) -> (bool, [u8; 2]) {
        if depth == 4 {
            if let SnailfishElem::Pair(snailfish) = self {
                if let Snailfish([SnailfishElem::Regular(a), SnailfishElem::Regular(b)]) = **snailfish {
                    *self = SnailfishElem::Regular(0);
                    return (true, [a, b]);
                }
            }
        } else if let SnailfishElem::Pair(snailfish) = self {
            return snailfish.try_explode_recur(depth);
        }
        (false, [0, 0])
    }

    fn try_split(&mut self) -> bool {
        match self {
            SnailfishElem::Regular(num) => {
                if *num >= 10 {
                    let a = *num / 2;
                    let b = (*num + 1) / 2;
                    *self = SnailfishElem::Pair(Box::new(
                            Snailfish([
                                SnailfishElem::Regular(a),
                                SnailfishElem::Regular(b),
                            ])));
                    true
                } else {
                    false
                }
            },
            SnailfishElem::Pair(snailfish) => snailfish.try_split(),
        }
    }

    fn add_to_rightmost(&mut self, val: u8) {
        match self {
            SnailfishElem::Regular(num) => *num += val,
            SnailfishElem::Pair(snailfish) =>
                snailfish.0[1].add_to_rightmost(val),
        }
    }

    fn add_to_leftmost(&mut self, val: u8) {
        match self {
            SnailfishElem::Regular(num) => *num += val,
            SnailfishElem::Pair(snailfish) =>
                snailfish.0[0].add_to_leftmost(val),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishElem::Regular(num) => *num as u32,
            SnailfishElem::Pair(snailfish) => snailfish.magnitude(),
        }
    }

}

impl Add for Snailfish {
    type Output = Snailfish;
    fn add(self, rhs: Self) -> Self {
        let mut sum = Snailfish([
            SnailfishElem::Pair(Box::new(self)),
            SnailfishElem::Pair(Box::new(rhs)),
        ]);
        sum.reduce();
        sum
    }
}

#[allow(dead_code)]
impl fmt::Display for SnailfishElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailfishElem::Regular(num) => write!(f, "{}", num),
            SnailfishElem::Pair(inner) => write!(f, "{}", inner),
        }
    }
}


#[allow(dead_code)]
impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        write!(f, "{}", self.0[0])?;
        write!(f, ",")?;
        write!(f, "{}", self.0[1])?;
        write!(f, "]")
    }
}

fn part1(puzzle_input: &str) -> u32 {
    let mut sum = Snailfish::parse(puzzle_input.lines().next().unwrap());
    for s in puzzle_input.lines().skip(1) {
        sum = sum + Snailfish::parse(s);
    }
    sum.magnitude()
}

fn part2(puzzle_input: &str) -> u32 {
    let snailfishes: Vec<_> = puzzle_input.lines().map(|s| Snailfish::parse(s)).collect();
    let mut best_magnitude = 0;
    for i in 0..snailfishes.len() {
        for j in 0..snailfishes.len() {
            if i == j { continue }
            let mag = (snailfishes[i].clone() + snailfishes[j].clone()).magnitude();
            if mag > best_magnitude {
                best_magnitude = mag;
            }
        }
    }
    best_magnitude
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_display() {
        let check = |s| {
            let snailfish = Snailfish::parse(s);
            let s2 = format!("{}", snailfish);
            assert_eq!(s, &s2);
        };
        check("[1,2]");
        check("[[1,2],3]");
        check("[9,[8,7]]");
        check("[[1,9],[8,5]]");
        check("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        check("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        check("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn test_explode() {
        let check = |s, expected| {
            let mut s = Snailfish::parse(s);
            s.reduce();
            let s = format!("{}", s);
            assert_eq!(s, expected);
        };

        check("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        check("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        check("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        check("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        check("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_example_add() {
        let s1 = Snailfish::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let s2 = Snailfish::parse("[1,1]");
        let s = format!("{}", s1 + s2);
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", s);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(29, Snailfish::parse("[9,1]").magnitude());
        assert_eq!(21, Snailfish::parse("[1,9]").magnitude());
        assert_eq!(129, Snailfish::parse("[[9,1],[1,9]]").magnitude());
        assert_eq!(143, Snailfish::parse("[[1,2],[[3,4],5]]").magnitude());
        assert_eq!(29, Snailfish::parse("[9,1]").magnitude());
        assert_eq!(1384, Snailfish::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude());
        assert_eq!(445, Snailfish::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude());
        assert_eq!(791, Snailfish::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude());
        assert_eq!(1137, Snailfish::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude());
        assert_eq!(3488, Snailfish::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude());
    }

    const EX: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part1() {
        assert_eq!(4140, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3993, part2(EX));
    }
}
