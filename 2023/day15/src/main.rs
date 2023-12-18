use std::io::{self, Read};

fn hash(s: &str) -> u32 {
    let mut curr = 0;
    for ch in s.chars() {
        curr += ch as u32;
        curr *= 17;
        curr %= 256;
    }
    curr
}

fn part1(puzzle_input: &str) -> u32 {
    puzzle_input.split(",").map(|s| hash(s)).sum()
}

fn part2(puzzle_input: &str) -> u32 {
    let mut boxes = vec![Vec::new(); 256];
    for s in puzzle_input.split(",") {
        let op_idx = s.find(|ch| ch == '-' || ch == '=').unwrap();
        let op = &s[op_idx..=op_idx];
        let label = &s[0..op_idx];
        let box_num = hash(label) as usize;
        match op {
            "-" => {
                boxes[box_num].retain(|(l, _)| l != &label);
            },
            "=" => {
                let focal_len: u32 = s[(op_idx+1)..s.len()].parse().unwrap();
                if let Some(idx) = boxes[box_num].iter().position(|(l, _)| l == &label) {
                    boxes[box_num][idx].1 = focal_len;
                } else {
                    boxes[box_num].push((label, focal_len));
                }
            },
            _ => panic!(),
        }
    }
    let mut sum = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (j, (_label, focal_len)) in b.into_iter().enumerate() {
            sum += (i as u32 + 1) * (j as u32 + 1) * focal_len;
        }
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let puzzle_input = puzzle_input.trim_end();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 145);
    }
}
