use std::collections::BTreeMap;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn parse_row(row: &str) -> Vec<bool> {
    row.chars()
        .map(|ch| {
            match ch {
                '#' => true,
                '.' => false,
                _ => unreachable!()
            }
        })
        .collect()
}
fn parse2(top: &str, bottom: &str) -> Vec<bool> {
    let mut ret = parse_row(top);
    ret.append(&mut parse_row(bottom));
    ret
}
fn parse3(top: &str, middle: &str, bottom: &str) -> Vec<bool> {
    let mut ret = parse_row(top);
    ret.append(&mut parse_row(middle));
    ret.append(&mut parse_row(bottom));
    ret
}
fn parse4(top: &str, topmid: &str, botmid: &str, bottom: &str) -> Vec<bool> {
    let mut ret = parse_row(top);
    ret.append(&mut parse_row(topmid));
    ret.append(&mut parse_row(botmid));
    ret.append(&mut parse_row(bottom));
    ret
}

fn parse(input: &str) -> BTreeMap<Vec<bool>, Vec<bool>> {
    lazy_static!{
        static ref RE2: Regex = Regex::new("^([\\.#]{2})/([\\.#]{2}) => ([\\.#]{3})/([\\.#]{3})/([\\.#]{3})$").unwrap();
        static ref RE3: Regex = Regex::new("^([\\.#]{3})/([\\.#]{3})/([\\.#]{3}) => ([\\.#]{4})/([\\.#]{4})/([\\.#]{4})/([\\.#]{4})$").unwrap();
    }
    input.lines()
        .map(|line| {
            if let Some(cap) = RE2.captures(line) {
                (parse2(&cap[1], &cap[2]), parse3(&cap[3], &cap[4], &cap[5]))
            } else if let Some(cap) = RE3.captures(line) {
                (parse3(&cap[1], &cap[2], &cap[3]), parse4(&cap[4], &cap[5], &cap[6], &cap[7]))
            } else {
                unreachable!()
            }
        })
        .collect()
}


fn extract(block_x: usize, block_y: usize, image: &Vec<bool>, size: usize, step: usize) -> Vec<bool> {
    let mut ret = Vec::new();
    for y in block_y*step .. (block_y+1)*step {
        for x in block_x*step .. (block_x+1)*step {
            ret.push(image[y*size + x]);
        }
    }
    ret
}

fn putback(block_x: usize, block_y: usize, block: Vec<bool>, image: &mut Vec<bool>, size: usize, step: usize) {
    let mut block_iter = block.into_iter();
    for y in block_y*step .. (block_y+1)*step {
        for x in block_x*step .. (block_x+1)*step {
            image[y*size + x] = block_iter.next().unwrap();
        }
    }
    assert!(block_iter.next().is_none());
}

fn step_for(block: &Vec<bool>) -> usize {
    if block.len() == 4 { 2 } else { 3 }
}

// aa -> bb
// bb -> aa
fn flip_horizontal(block: &Vec<bool>) -> Vec<bool> {
    let step = step_for(block);
    let mut ret = vec![false; block.len()];
    for y in 0..step {
        for x in 0..step {
            ret[(step - y - 1)*step + x] = block[y*step + x];
        }
    }
    ret
}

// ab -> ba
// ab -> ba
fn flip_vertical(block: &Vec<bool>) -> Vec<bool> {
    let step = step_for(block);
    let mut ret = vec![false; block.len()];
    for y in 0..step {
        for x in 0..step {
            ret[y*step + (step - x - 1)] = block[y*step + x];
        }
    }
    ret
}

fn rotate_left(block: &Vec<bool>) -> Vec<bool> {
    if step_for(block) == 2 {
        vec![block[1], block[3],
             block[0], block[2]]
    } else {
        vec![block[2], block[5], block[8],
             block[1], block[4], block[7],
             block[0], block[3], block[6]]
    }
}

fn rotate_right(block: &Vec<bool>) -> Vec<bool> {
    if step_for(block) == 2 {
        vec![block[2], block[0],
             block[3], block[1]]
    } else {
        vec![block[6], block[3], block[0],
             block[7], block[4], block[1],
             block[8], block[5], block[2]]
    }
}

#[allow(dead_code)]
fn print_block(block: &Vec<bool>) -> String {
    let step = step_for(block);
    let mut ret = String::new();
    for (i, bit) in block.iter().enumerate() {
        if *bit {
            ret.push('#');
        } else {
            ret.push('.');
        }
        if i != step * step - 1 && i % step == step - 1 {
            ret.push('/');
        }
    }
    ret
}

fn convert(block: Vec<bool>, rules: &BTreeMap<Vec<bool>, Vec<bool>>) -> Vec<bool> {
    for (input, output) in rules.iter() {
        if input == &block {
            return output.clone();
        } else if input == &flip_horizontal(&block) {
            return output.clone();
        } else if input == &flip_vertical(&block) {
            return output.clone();
        } else if input == &flip_horizontal(&flip_vertical(&block))  {
            return output.clone();
        } else if input == &rotate_left(&block) {
            return output.clone();
        } else if input == &rotate_right(&block) {
            return output.clone();
        } else if input == &rotate_left(&flip_vertical(&block)) {
            return output.clone();
        } else if input == &rotate_right(&flip_vertical(&block)) {
            return output.clone();
        }
    }
    unreachable!()
}

fn calc(input: &str, steps: usize) -> usize {
    let rules = parse(input);
    let mut image =
        vec![false, true,  false,
             false, false, true,
             true,  true,  true];
    let mut size = 3;

    for _ in 0..steps {
        if size % 2 == 0 {
            let newsize = size/2*3;
            let mut newimage = vec![false; newsize * newsize];
            for block_y in 0..size/2 {
                for block_x in 0..size/2 {
                    let block = extract(block_x, block_y, &image, size, 2);
                    let block = convert(block, &rules);
                    putback(block_x, block_y, block, &mut newimage, newsize, 3);
                }
            }
            size = newsize;
            image = newimage;
        } else {
            let newsize = size/3*4;
            let mut newimage = vec![false; newsize * newsize];
            for block_y in 0..size/3 {
                for block_x in 0..size/3 {
                    let block = extract(block_x, block_y, &image, size, 3);
                    let block = convert(block, &rules);
                    putback(block_x, block_y, block, &mut newimage, newsize, 4);
                }
            }
            size = newsize;
            image = newimage;
        }
    }

    image.iter().filter(|bit| **bit).count()
}

fn part1(input: &str) -> usize {
    calc(input, 5)
}

fn part2(input: &str) -> usize {
    calc(input, 18)
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
        let ex = "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        assert_eq!(calc(ex, 2), 12);
    }
}
