use std::io::{self, Read};
use std::collections::HashMap;
use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Dir {
    dx: isize,
    dy: isize,
}

fn parse(puzzle_input: &str) -> Vec<(Pos, Dir)> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    puzzle_input.lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let pos = Pos {
            x: cap[1].parse::<usize>().unwrap(),
            y: cap[2].parse::<usize>().unwrap(),
        };
        let dir = Dir {
            dx: cap[3].parse::<isize>().unwrap(),
            dy: cap[4].parse::<isize>().unwrap(),
        };
        (pos, dir)
    }).collect()
}

fn part1(robots: &[(Pos, Dir)], width: usize, height: usize) -> usize {
    let mut positions = Vec::new();
    for robot in robots {
        positions.push(Pos {
            x: (robot.0.x + 100 * (width as isize + robot.1.dx) as usize) % width,
            y: (robot.0.y + 100 * (height as isize + robot.1.dy) as usize) % height,
        });
    }
    let mut quadrants = vec![0usize; 4];
    for pos in &positions {
        if pos.y < height / 2 {
            if pos.x < width / 2 {
                quadrants[0] += 1;
            } else if pos.x > width / 2 {
                quadrants[1] += 1;
            }
        } else if pos.y > height / 2 {
            if pos.x < width / 2 {
                quadrants[2] += 1;
            } else if pos.x > width / 2 {
                quadrants[3] += 1;
            }
        }
    }
    quadrants.iter().cloned().product()
}

#[allow(unused)]
fn print_image(positions: &HashMap<Pos, usize>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let pos = Pos { x, y };
            if let Some(cnt) = positions.get(&pos) {
                if *cnt > 9 {
                    print!("*");
                } else {
                    print!("{}", *cnt);
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part2(robots: &[(Pos, Dir)]) -> usize {
    let width = 101;
    let height = 103;
    for iters in 1..(width * height) {
        let mut positions = HashMap::new();
        for (pos, dir) in robots {
            positions.entry(Pos {
                x: (pos.x + iters * (width as isize + dir.dx) as usize) % width,
                y: (pos.y + iters * (height as isize + dir.dy) as usize) % height,
            }).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        // If enough robots are next to each other, that's the easter egg
        let mut contiguous = 0usize;
        for (pos, _) in &positions {
            if pos.x > 0 && positions.contains_key(&Pos { x: pos.x - 1, y: pos.y }) { contiguous += 1; }
            if pos.y > 0 && positions.contains_key(&Pos { x: pos.x, y: pos.y - 1 }) { contiguous += 1; }
        }
        if contiguous >= robots.len() {
            // print_image(&positions, width, height);
            return iters;
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let robots = parse(&puzzle_input);
    println!("{}", part1(&robots, 101, 103));
    println!("{}", part2(&robots));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX), 11, 7), 12);
    }
}
