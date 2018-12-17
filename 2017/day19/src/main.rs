use std::io::{self, Read};

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    for row in matrix.iter() {
        assert_eq!(row.len(), matrix[0].len());
    }

    matrix
}

fn start_x(matrix: &Vec<Vec<char>>) -> usize {
    for (i,ch) in matrix[0].iter().enumerate() {
        if *ch == '|' {
            return i
        }
    }
    unreachable!()
}

fn step(x: &mut usize, y: &mut usize, dir_x: isize, dir_y: isize) {
    *x = (*x as isize + dir_x) as usize;
    *y = (*y as isize + dir_y) as usize;
}

fn solve(input: &str) -> (String, usize) {
    let matrix = parse(input);
    let mut path = String::new();

    let width = matrix[0].len();
    let height = matrix.len();
    let mut x = start_x(&matrix);
    let mut y = 0;
    let mut dir_x = 0;
    let mut dir_y = 1;
    let mut steps = 0;

    loop {
        match matrix[y][x] {
            '|' | '-' => step(&mut x, &mut y, dir_x, dir_y),
            ch @ 'A'...'Z' => {
                path.push(ch);
                step(&mut x, &mut y, dir_x, dir_y);
            },
            ' ' => break,
            '+' => {
                if dir_x != 0 && dir_y != 1 && y+1 < height && matrix[y+1][x] != ' ' {
                    dir_x = 0; dir_y = 1;
                } else if dir_x != 0 && dir_y != -1 && y > 0 && matrix[y-1][x] != ' ' {
                    dir_x = 0; dir_y = -1;
                } else if dir_x != 1 && dir_y != 0 && x+1 < width && matrix[y][x+1] != ' ' {
                    dir_x = 1; dir_y = 0;
                } else if dir_x != -1 && dir_y != 0 && x > 0 && matrix[y][x-1] != ' ' {
                    dir_x = -1; dir_y = 0;
                } else {
                    unreachable!()
                }
                step(&mut x, &mut y, dir_x, dir_y);
            },
            _ => unreachable!()
        }
        steps += 1;
    }

    (path, steps)
}

fn part1(input: &str) -> String {
    solve(input).0
}

fn part2(input: &str) -> usize {
    solve(input).1
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

    const EX: &str =
"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), "ABCDEF".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 38);
    }

}
