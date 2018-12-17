use std::collections::BTreeMap;
use std::io::{self, Read};

fn grid_size(square: usize) -> usize {
    for i in 0.. {
        if square <= (2*i + 1) * (2*i + 1) {
            return i
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    let square = input.trim_end().parse().unwrap();
    let width = grid_size(square);

    let mut dir: i32 = -1;
    let mut cnt = 0;
    let mut dist = 2 * width;
    for _ in (square .. (2*width + 1)*(2*width + 1)).rev() {
        cnt += 1;
        if dir < 0 { dist -= 1; } else { dist += 1 }
        if cnt >= width {
            cnt = 0;
            dir *= -1;
        }
    }

    dist
}

fn get_adjacent_sum(x: i32, y: i32, grid: &BTreeMap<(i32, i32), usize>) -> usize {
    let mut sum = 0;
    sum += grid.get(&(x+1,y+1)).unwrap_or(&0);
    sum += grid.get(&(x+1,y)).unwrap_or(&0);
    sum += grid.get(&(x+1,y-1)).unwrap_or(&0);
    sum += grid.get(&(x,y+1)).unwrap_or(&0);
    sum += grid.get(&(x,y-1)).unwrap_or(&0);
    sum += grid.get(&(x-1,y+1)).unwrap_or(&0);
    sum += grid.get(&(x-1,y)).unwrap_or(&0);
    sum += grid.get(&(x-1,y-1)).unwrap_or(&0);
    sum
}

fn set_grid_value(x: i32, y: i32, grid: &mut BTreeMap<(i32, i32), usize>) -> usize {
    let val = get_adjacent_sum(x, y, grid);
    grid.insert((x, y), val);
    val
}

fn part2(input: &str) -> usize {
    let input_int: usize = input.trim_end().parse().unwrap();
    let mut grid: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    grid.insert((0,0), 1);
    for i in 1.. {
        for y in (1-i) ..= (i-1) { let val = set_grid_value(i, y, &mut grid); if val > input_int { return val } }
        for x in (-i ..= i).rev() { let val = set_grid_value(x, i, &mut grid); if val > input_int { return val } }
        for y in ((1-i) ..= (i-1)).rev() { let val = set_grid_value(-i, y, &mut grid); if val > input_int { return val } }
        for x in -i ..= i { let val = set_grid_value(x, -i, &mut grid); if val > input_int { return val } }
    }
    unreachable!();
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
        assert_eq!(part1("1"), 0);
        assert_eq!(part1("12"), 3);
        assert_eq!(part1("23"), 2);
        assert_eq!(part1("1024"), 31);
    }

}
