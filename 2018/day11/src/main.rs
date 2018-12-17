use std::io::{self, Read};
use std::ops::RangeInclusive;
use std::cmp::{min, max};

fn power_level(x: usize, y: usize, grid_serial: usize) -> i32 {
    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + grid_serial;
    let power = power * rack_id;
    let power = (power / 100) % 10; // hundreds place
    let power = power as i32 - 5;
    power
}

fn idx(x: usize, y: usize) -> usize {
    debug_assert!(x > 0 && x <= 300 && y > 0 && y <= 300);
    300 * (x-1) + y-1
}

fn build_grid(grid_serial: usize) -> Vec<i32> {
    let mut grid = vec![0; 300 * 300];
    for x in 1..=300 {
        for y in 1..=300 {
            grid[idx(x, y)] = power_level(x, y, grid_serial);
        }
    }
    grid
}

// returns (x, y, size)
fn best_square(grid: &Vec<i32>, square_sizes: RangeInclusive<usize>) -> (usize, usize, usize) {
    let mut best_sum = std::i32::MIN;
    let mut best_xys = (1,1,0);
    for topleft_x in 1 ..= 300 {
        for topleft_y in 1 ..= 300 {
            let mut sum = 0;
            for square_size in 1 ..= min(301 - max(topleft_x, topleft_y), *square_sizes.end()) {
                for x in topleft_x .. topleft_x + square_size - 1 {
                    sum += grid[idx(x, topleft_y + square_size - 1)];
                }
                for y in topleft_y ..= topleft_y + square_size - 1 {
                    sum += grid[idx(topleft_x + square_size - 1, y)];
                }

                if sum > best_sum && *square_sizes.start() <= square_size {
                    best_sum = sum;
                    best_xys = (topleft_x, topleft_y, square_size);
                }
            }
        }
    }

    best_xys
}

fn part1(input: &str) -> String {
    let grid_serial = input.parse().unwrap();
    let grid = build_grid(grid_serial);
    let (x, y, _) = best_square(&grid, 3..=3);
    format!("{},{}", x, y)
}

fn part2(input: &str) -> String {
    let grid_serial = input.parse().unwrap();
    let grid = build_grid(grid_serial);
    let (x, y, size) = best_square(&grid, 1..=300);
    format!("{},{},{}", x, y, size)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input.trim_end()));
    println!("{}", part2(&input.trim_end()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level_examples() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("18"), "33,45".to_string());
        assert_eq!(part1("42"), "21,61".to_string());
    }

    #[test]
    fn test_part2() {
        // Need to use --release to test this since it takes a while
        assert_eq!(part2("18"), "90,269,16".to_string());
        assert_eq!(part2("42"), "232,251,12".to_string());
    }

}
