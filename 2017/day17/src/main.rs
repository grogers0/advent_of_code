use std::io::{self, Read};
use std::ops::Index;

enum TreeVec<T> {
    Leaf(Vec<T>),
    Internal(usize, Vec<Box<TreeVec<T>>>)
}

impl <T> TreeVec<T> {
    fn new() -> TreeVec<T> {
        TreeVec::Leaf(Vec::new())
    }

    fn len(&self) -> usize {
        match self {
            TreeVec::Leaf(elems) => elems.len(),
            TreeVec::Internal(len, _) => *len
        }
    }

    // OK  this is totally messed up but because of the way things are inserted randomly it's good
    // enough for the 100x challenge
    fn insert(&mut self, idx: usize, elem: T) {
        const MAX: usize = 100;
        match self {
            TreeVec::Leaf(elems) => {
                elems.insert(idx, elem);
                if elems.len() == MAX {
                    let elems2 = elems.split_off(MAX/2);
                    let elems1 = elems.split_off(0);
                    *self = TreeVec::Internal(MAX, vec![
                        Box::new(TreeVec::Leaf(elems1)),
                        Box::new(TreeVec::Leaf(elems2))]);
                }
            },
            TreeVec::Internal(ref mut len, children) => {
                let mut cnt = 0;
                assert!(idx < *len);
                for i in 0..children.len() {
                    if cnt + children[i].len() > idx {
                        *len += 1;
                        children[i].insert(idx - cnt, elem);
                        return;
                    } else {
                        cnt += children[i].len();
                    }
                }
                unreachable!();
            }
        }
    }
}

impl <T> Index<usize> for TreeVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        match self {
            TreeVec::Leaf(elems) => &elems[idx],
            TreeVec::Internal(len, children) => {
                let mut cnt = 0;
                assert!(idx < *len);
                for i in 0..children.len() {
                    if cnt + children[i].len() > idx {
                        return &children[i][idx - cnt];
                    } else {
                        cnt += children[i].len();
                    }
                }
                unreachable!();
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let step_size: usize = input.trim_end().parse().unwrap();
    let mut buffer = Vec::new();
    buffer.push(0);

    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + step_size + 1) % buffer.len();
        buffer.insert(pos, i);
    }
    buffer[(pos + 1) % buffer.len()]
}

fn part2(input: &str) -> usize {
    let step_size: usize = input.trim_end().parse().unwrap();
    let mut buffer = TreeVec::new();
    buffer.insert(0, 0);

    let mut pos = 0;
    for i in 1..=50000000 {
        pos = (pos + step_size + 1) % buffer.len();
        buffer.insert(pos, i);
    }
    for i in 0..buffer.len() {
        if buffer[i] == 0 {
            pos = i;
            break;
        }
    }
    buffer[(pos + 1) % buffer.len()]
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
        assert_eq!(part1("3"), 638);
    }
}
