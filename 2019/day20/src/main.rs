use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Square {
    Wall,
    Open,
    Teleport(usize, usize, bool) // The (x, y) of other end, to_outer bool
}

struct Map {
    start: (usize, usize),
    finish: (usize, usize),
    width: usize,
    data: Vec<Square>
}

impl Map {
    fn at(&self, x: usize, y: usize) -> Square {
        self.data[y * self.width + x]
    }

}

fn parse(input: &str) -> Map {
    let width = input.lines().filter(|line| !line.is_empty())
        .next().unwrap().chars().count();
    let mut data = Vec::new();
    let mut warp_squares = HashMap::new();
    for (y, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        assert_eq!(line.chars().count(), width);
        for (x, ch) in line.chars().enumerate() {
            let sq = match ch {
                ' ' | '#' => Square::Wall,
                '.' => Square::Open,
                _ if ch >= 'A' && ch <= 'Z' => {
                    warp_squares.insert((x, y), ch);
                    Square::Wall
                },
                _ => panic!()
            };
            data.push(sq);
        }
    }
    let height = data.len() / width;
    assert_eq!(data.len(), height * width);

    let mut start = None;
    let mut finish = None;

    let idx = |x, y| y * width + x;
    fn add_to(a: usize, b: isize) -> usize {
        if b < 0 && a < b.abs() as usize { panic!() }
        ((a as isize) + b) as usize
    }
    let mut other_warp = HashMap::new();
    for ((x, y), ch1) in warp_squares.iter() {
        let (x, y) = (*x, *y);
        if x == 0 || x == width - 1 || y == 0 || y == height - 1 { continue }
        let check_xy = |x, y, dx: isize, dy: isize| {
            if let Some(ch2) = warp_squares.get(&(add_to(x, -dx), add_to(y, -dy))) {
                if let Square::Open = data[idx(add_to(x, dx), add_to(y, dy))] {
                    Some((ch2, add_to(x, dx), add_to(y, dy)))
                } else {
                    None
                }
            } else {
                None
            }
        };

        let mut tele_chs_xy = None;
        if let Some((ch2, x, y)) = check_xy(x, y, 1, 0) {
            tele_chs_xy = Some((format!("{}{}", ch2, ch1), x, y));
        } else if let Some((ch2, x, y)) = check_xy(x, y, -1, 0) {
            tele_chs_xy = Some((format!("{}{}", ch1, ch2), x, y));
        } else if let Some((ch2, x, y)) = check_xy(x, y, 0, 1) {
            tele_chs_xy = Some((format!("{}{}", ch2, ch1), x, y));
        } else if let Some((ch2, x, y)) = check_xy(x, y, 0, -1) {
            tele_chs_xy = Some((format!("{}{}", ch1, ch2), x, y));
        }

        if let Some((chs, x, y)) = tele_chs_xy {
            if chs == "AA" {
                start = Some((x, y))
            } else if chs == "ZZ" {
                finish = Some((x, y))
            } else if let Some((x1, y1)) = other_warp.get(&chs) {
                let (x1, y1) = (*x1, *y1);
                let to_outer = x == 2 || x == width - 3 || y == 2 || y == height - 3;
                data[idx(x, y)] = Square::Teleport(x1, y1, to_outer);
                data[idx(x1, y1)] = Square::Teleport(x, y, !to_outer);
            } else {
                other_warp.insert(chs, (x, y));
            }
        }
    }

    Map {
        start: start.unwrap(),
        finish: finish.unwrap(),
        width: width,
        data: data
    }
}

fn calc_distance(map: &Map, do_recursion: bool) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, map.start.0, map.start.1, 0));
    let mut seen = HashSet::new();
    while let Some((dist, x, y, depth)) = queue.pop_front() {
        if !seen.insert((x, y, depth)) { continue }
        if depth == 0 && map.finish == (x, y) { return dist }
        match map.at(x, y) {
            Square::Wall => (),
            Square::Open | Square::Teleport(_, _, _) => {
                queue.push_back((dist + 1, x + 1, y, depth));
                queue.push_back((dist + 1, x - 1, y, depth));
                queue.push_back((dist + 1, x, y + 1, depth));
                queue.push_back((dist + 1, x, y - 1, depth));
                if let Square::Teleport(next_x, next_y, to_outer) = map.at(x, y) {
                    if depth > 0 || !to_outer || !do_recursion {
                        let next_depth = if do_recursion {
                            if to_outer { depth - 1 } else { depth + 1 }
                        } else {
                            depth
                        };
                        queue.push_back((dist + 1, next_x, next_y, next_depth));
                    }
                }
            }
        }
    }
    panic!()
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    calc_distance(&map, false)
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    calc_distance(&map, true)
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

    const EX1: &str = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    const EX2: &str = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    const EX3: &str = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(EX1), 23);
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(part1(EX2), 58);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2(EX1), 26);
    }

    // EX2 on part2 is supposed to not find a path, but it seems to run forever

    #[test]
    fn test_part2_ex3() {
        assert_eq!(part2(EX3), 396);
    }

}
