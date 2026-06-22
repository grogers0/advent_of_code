use std::io::{self, Read};

fn part1(puzzle_input: &str) -> usize {
    let width = puzzle_input.lines().next().unwrap().len();
    let height = puzzle_input.lines().count();
    let expected_len = (width + 1) * height;
    assert!(expected_len == puzzle_input.len() || expected_len == puzzle_input.len() + 1);
    let idx = |x: usize, y: usize| -> usize { (width + 1) * y + x };
    const OFFSETS: [(isize, isize); 8] = [
        (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    const XMAS: &str = "XMAS";
    let check = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        if dx == 1  && x + 3 >= width  { return false; }
        if dx == -1 && x < 3           { return false; }
        if dy == 1  && y + 3 >= height { return false; }
        if dy == -1 && y < 3           { return false; }
        for i in 1..4 {
            let x2 = (x as isize + i * dx) as usize;
            let y2 = (y as isize + i * dy) as usize;
            if &puzzle_input[idx(x2, y2)..=idx(x2, y2)] != &XMAS[i as usize..=i as usize] {
                return false;
            }
        }
        true
    };


    let mut cnt = 0;
    for y in 0..height {
        for x in 0..width {
            if &puzzle_input[idx(x, y)..=idx(x, y)] != "X" { continue; }
            for &(dx, dy) in &OFFSETS {
                if check(x, y, dx, dy) { cnt += 1; }
            }
        }
    }
    cnt
}

fn part2(puzzle_input: &str) -> usize {
    let width = puzzle_input.lines().next().unwrap().len();
    let height = puzzle_input.lines().count();
    let expected_len = (width + 1) * height;
    assert!(expected_len == puzzle_input.len() || expected_len == puzzle_input.len() + 1);
    let idx = |x: usize, y: usize| -> usize { (width + 1) * y + x };
    let check = |x: usize, y: usize| -> bool {
        let mut seen_s = false;
        let mut seen_m = false;
        let mut seen_both_s = false;
        let mut seen_both_m = false;
        let mut first_s = (0isize, 0isize);

        const OFFSETS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
        for &(dx, dy) in &OFFSETS {
            let x2 = (x as isize + dx) as usize;
            let y2 = (y as isize + dy) as usize;
            match &puzzle_input[idx(x2, y2)..=idx(x2, y2)] {
                "S" => {
                    if seen_both_s {
                        return false;
                    } else if seen_s {
                        seen_both_s = true;
                        if dx != first_s.0 && dy != first_s.1 {
                            return false;
                        }
                    } else {
                        seen_s = true;
                        first_s = (dx, dy);
                    }
                },
                "M" => {
                    if seen_both_m {
                        return false;
                    } else if seen_m {
                        seen_both_m = true;
                    } else {
                        seen_m = true;
                    }
                },
                _ => return false,
            }
        }
        seen_both_s && seen_both_m
    };


    let mut cnt = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if &puzzle_input[idx(x, y)..=idx(x, y)] != "A" { continue; }
            if check(x, y) { cnt += 1; }
        }
    }
    cnt
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

    const EX: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 9);
    }
}
