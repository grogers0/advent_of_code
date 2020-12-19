use std::io::{self, Read};

#[derive(PartialEq)]
enum Token {
    Num(u64),
    Plus,
    Star,
    LParen,
    RParen
}

enum Op {
    Add,
    Mul
}

enum Precedence {
    LeftToRight,
    AddBeforeMul
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // NOTE - input doesn't have any numbers larger than 9 so we don't need to handle them
    for ch in line.chars() {
        let token = match ch {
            '0'..='9' => Some(Token::Num(ch as u64 - '0' as u64)),
            '+' => Some(Token::Plus),
            '*' => Some(Token::Star),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ' ' => None,
            _ => panic!()
        };
        token.map(|t| tokens.push(t));
    }
    tokens
}

fn eval(tokens: &[Token], prec: &Precedence) -> u64 {
    fn value(tokens: &[Token], idx: &mut usize, prec: &Precedence) -> u64 {
        match tokens[*idx] {
            Token::Num(n) => { *idx += 1; n },
            Token::LParen => { *idx += 1; inner(tokens, idx, prec, true) },
            _ => panic!()
        }
    }

    fn operation(tokens: &[Token], idx: &mut usize) -> Op {
        match tokens[*idx] {
            Token::Plus => { *idx += 1; Op::Add },
            Token::Star => { *idx += 1; Op::Mul },
            _ => panic!()
        }
    }

    fn inner(tokens: &[Token], idx: &mut usize, prec: &Precedence, expect_rparen: bool) -> u64 {
        let mut v1 = value(tokens, idx, prec);
        loop {
            if expect_rparen && Token::RParen == tokens[*idx] { *idx += 1; return v1; }
            if !expect_rparen && *idx == tokens.len() { return v1; }

            let op = operation(tokens, idx);
            match (prec, &op) {
                (Precedence::LeftToRight, _) | (Precedence::AddBeforeMul, Op::Add) => {
                    let v2 = value(tokens, idx, prec);
                    match op {
                        Op::Add => v1 += v2,
                        Op::Mul => v1 *= v2
                    };
                },
                (Precedence::AddBeforeMul, Op::Mul) => {
                    let v2 = inner(tokens, idx, prec, expect_rparen);
                    return v1 * v2;
                }
            }
        }
    }

    let mut idx = 0;
    inner(tokens, &mut idx, prec, false)
}

fn part1(puzzle_input: &str) -> u64 {
    puzzle_input.lines().map(|line| eval(&tokenize(line), &Precedence::LeftToRight)).sum()
}

fn part2(puzzle_input: &str) -> u64 {
    puzzle_input.lines().map(|line| eval(&tokenize(line), &Precedence::AddBeforeMul)).sum()
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
    fn test_part1() {
        assert_eq!(71, part1("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, part1("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, part1("2 * 3 + (4 * 5)"));
        assert_eq!(437, part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(13632, part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(231, part2("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, part2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, part2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669060, part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(23340, part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}
