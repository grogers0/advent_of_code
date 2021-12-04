use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

#[derive(Clone)]
struct BingoBoard {
    nums: HashMap<u8, (usize, usize)>, // num -> (row, col)
    marked_nums: HashSet<u8>,
    marked_rows: [usize; BingoBoard::SIDE],
    marked_cols: [usize; BingoBoard::SIDE],
}

impl BingoBoard {
    const SIDE: usize = 5;

    fn parse(input: &str) -> BingoBoard {
        let mut nums = HashMap::with_capacity(Self::SIDE * Self::SIDE);
        for (i, s) in input.split_whitespace().enumerate() {
            let row = i / Self::SIDE;
            let col = i % Self::SIDE;
            nums.insert(s.parse().unwrap(), (row, col));
        }
        assert_eq!(Self::SIDE * Self::SIDE, nums.len());
        BingoBoard {
            nums,
            marked_nums: HashSet::with_capacity(Self::SIDE * Self::SIDE),
            marked_rows: [0; Self::SIDE],
            marked_cols: [0; Self::SIDE],
        }
    }

    // Return true if we are now a winner
    fn mark(&mut self, num: u8) -> bool {
        if !self.nums.contains_key(&num) { return false }
        if self.marked_nums.insert(num) {
            let (row, col) = self.nums[&num];
            self.marked_rows[row] += 1;
            self.marked_cols[col] += 1;
            self.marked_rows[row] == Self::SIDE || self.marked_cols[col] == Self::SIDE
        } else {
            false
        }
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for num in self.nums.keys() {
            if !self.marked_nums.contains(&num) {
                sum += *num as u32;
            }
        }
        sum
    }
}

fn parse(puzzle_input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut split_iter = puzzle_input.split("\n\n");
    let nums = split_iter.next().unwrap()
        .split(",").map(|s| s.parse().unwrap()).collect();
    let boards = split_iter.map(|s| BingoBoard::parse(s)).collect();
    (nums, boards)
}

fn part1(nums: &[u8], mut boards: Vec<BingoBoard>) -> u32 {
    for &num in nums {
        for board in boards.iter_mut() {
            if board.mark(num) {
                return board.sum_unmarked() * num as u32;
            }
        }
    }
    panic!()
}

fn part2(nums: &[u8], mut boards: Vec<BingoBoard>) -> u32 {
    let mut winning_boards = HashSet::new();
    let cnt_boards = boards.len();
    for &num in nums {
        for (i, board) in boards.iter_mut().enumerate() {
            if winning_boards.contains(&i) {
                continue;
            } else if board.mark(num) {
                winning_boards.insert(i);
                if winning_boards.len() == cnt_boards {
                    return board.sum_unmarked() * num as u32;
                }
            }
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (nums, boards) = parse(&puzzle_input);
    println!("{}", part1(&nums, boards.clone()));
    println!("{}", part2(&nums, boards.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let (nums, boards) = parse(EX);
        assert_eq!(4512, part1(&nums, boards));
    }

    #[test]
    fn test_part2() {
        let (nums, boards) = parse(EX);
        assert_eq!(1924, part2(&nums, boards));
    }
}
