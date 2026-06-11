use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

struct Problem {
    numbers: Vec<String>,
    op: Op,
}

fn parse(puzzle_input: &str) -> Vec<Problem> {
    let mut lines = puzzle_input.lines();
    let mut problems = vec![];
    let mut blanks = vec![];
    {
        let line = lines.next_back().unwrap();
        for (i, ch) in line.chars().enumerate() {
            if ch == ' ' {
                continue;
            }
            let op = match ch {
                '+' => Op::Add,
                '*' => Op::Mul,
                _ => panic!(),
            };
            problems.push(Problem { numbers: vec![], op });
            if i != 0 {
                blanks.push(i - 1);
            }
        }
        blanks.push(line.len());
        assert_eq!(blanks.len(), problems.len());
    }
    
    for line in lines {
        let mut j = 0;
        problems[j].numbers.push(String::new());
        for (i, ch) in line.chars().enumerate() {
            if blanks[j] == i {
                j += 1;
                problems[j].numbers.push(String::new());
            } else {
                problems[j].numbers.last_mut().unwrap().push(ch);
            }
        }
        assert_eq!(j + 1, problems.len());
    }
    for problem in problems.iter() {
        let str_len = problem.numbers.first().unwrap().len();
        for num in problem.numbers.iter() {
            assert_eq!(str_len, num.len());
        }
    }
    problems
}

fn calculate<I: Iterator<Item=u64>>(op: Op, numbers: I) -> u64 {
    match op {
        Op::Add => numbers.sum::<u64>(),
        Op::Mul => numbers.product::<u64>(),
    }
}

fn part1(problems: &[Problem]) -> u64 {
    let mut sum = 0;
    for problem in problems {
        sum += calculate(problem.op, problem.numbers.iter()
            .map(|s| s.trim().parse::<u64>().unwrap()));
    }
    sum
}

fn part2(problems: &[Problem]) -> u64 {
    let mut sum = 0;
    for problem in problems.iter() {
        let mut parsed_nums = vec![];
        for i in 0..problem.numbers.first().unwrap().len() {
            let mut num = 0;
            for s in problem.numbers.iter() {
                if &s[i..=i] != " " {
                    num *= 10;
                    num += s[i..=i].parse::<u64>().unwrap();
                }
            }
            parsed_nums.push(num);
        }
        sum += calculate(problem.op, parsed_nums.into_iter());
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let problems = parse(&puzzle_input);
    println!("{}", part1(&problems));
    println!("{}", part2(&problems));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 3263827);
    }
}
