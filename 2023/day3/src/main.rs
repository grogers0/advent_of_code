use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Cell {
    Blank, // denoted by a period
    Num(u32),
    Sym(char),
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn idx(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width && y < self.height);
        y * self.width + x
    }

    fn at(&self, x: usize, y: usize) -> Cell {
        self.cells[self.idx(x, y)]
    }
}

fn parse(puzzle_input: &str) -> Grid {
    let height = puzzle_input.lines().count();
    let width = puzzle_input.lines().next().unwrap().chars().count();
    let mut cells = Vec::with_capacity(width * height);
    for line in puzzle_input.lines() {
        assert_eq!(width, line.chars().count(), "input wasn't square");
        for ch in line.chars() {
            let cell = match ch {
                '.' => Cell::Blank,
                '0' ..= '9' => Cell::Num(ch as u32 - '0' as u32),
                _ => Cell::Sym(ch),

            };
            cells.push(cell);
        }
    }
    Grid { width, height, cells }
}

fn number_touches_any_symbol(grid: &Grid, x: usize, y: usize) -> bool {
    fn inner(grid: &Grid, x: usize, y: usize, seen: &mut [bool]) -> bool {
        if seen[grid.idx(x, y)] { return false }
        seen[grid.idx(x, y)] = true;
        match grid.at(x, y) {
            Cell::Blank  => return false,
            Cell::Sym(_) => return true,
            Cell::Num(_) => (),
        };
        // Are we touching the named edge
        let left   = x == 0;
        let right  = x == grid.width - 1;
        let top    = y == 0;
        let bottom = y == grid.height - 1;

        if !left  && !top    && inner(grid, x - 1, y - 1, seen) { return true }
        if !left             && inner(grid, x - 1, y,     seen) { return true }
        if !left  && !bottom && inner(grid, x - 1, y + 1, seen) { return true }
        if           !top    && inner(grid, x,     y - 1, seen) { return true }
        if           !bottom && inner(grid, x,     y + 1, seen) { return true }
        if !right && !top    && inner(grid, x + 1, y - 1, seen) { return true }
        if !right            && inner(grid, x + 1, y,     seen) { return true }
        if !right && !bottom && inner(grid, x + 1, y + 1, seen) { return true }
        false
    }
    let mut seen = vec![false; grid.width * grid.height];
    inner(grid, x, y, &mut seen)
}

fn number_starting_at(grid: &Grid, mut x: usize, y: usize) -> u32 {
    debug_assert!(matches!(grid.at(x, y), Cell::Num(_)));
    let mut num = 0;
    while x < grid.width {
        if let Cell::Num(digit) = grid.at(x, y) {
            num *= 10;
            num += digit;
            x += 1;
        } else {
            break;
        }
    }
    num
}

fn part1(grid: &Grid) -> u32 {
    let mut seen = vec![false; grid.width * grid.height];
    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if seen[grid.idx(x, y)] { continue }
            seen[grid.idx(x, y)] = true;
            if !matches!(grid.at(x, y), Cell::Num(_)) { continue }
            let mut x2 = x + 1;
            while x2 < grid.width && matches!(grid.at(x2, y), Cell::Num(_)) {
                seen[grid.idx(x2, y)] = true;
                x2 += 1;
            }
            if !number_touches_any_symbol(grid, x, y) { continue }
            sum += number_starting_at(grid, x, y);
        }
    }
    sum
}

fn touching_numbers(grid: &Grid, x: usize, y: usize) -> Vec<u32> {
    fn inner(grid: &Grid, x: usize, y: usize, seen: &mut [bool], ret: &mut Vec<u32>) {
        if seen[grid.idx(x, y)] { return }
        seen[grid.idx(x, y)] = true;
        if !matches!(grid.at(x, y), Cell::Num(_)) { return }
        // Fill seen for the whole number
        let mut x2 = x + 1;
        while x2 < grid.width && matches!(grid.at(x2, y), Cell::Num(_)) {
            seen[grid.idx(x2, y)] = true;
            x2 += 1;
        }
        x2 = x;
        while x2 > 0 && matches!(grid.at(x2 - 1, y), Cell::Num(_)) {
            seen[grid.idx(x2 - 1, y)] = true;
            x2 -= 1;
        }
        ret.push(number_starting_at(grid, x2, y));
    }

    // Are we touching the named edge
    let left   = x == 0;
    let right  = x == grid.width - 1;
    let top    = y == 0;
    let bottom = y == grid.height - 1;

    let mut seen = vec![false; grid.width * grid.height];
    let mut ret = Vec::new();
    if !left  && !top    { inner(grid, x - 1, y - 1, &mut seen, &mut ret) }
    if !left             { inner(grid, x - 1, y,     &mut seen, &mut ret) }
    if !left  && !bottom { inner(grid, x - 1, y + 1, &mut seen, &mut ret) }
    if           !top    { inner(grid, x,     y - 1, &mut seen, &mut ret) }
    if           !bottom { inner(grid, x,     y + 1, &mut seen, &mut ret) }
    if !right && !top    { inner(grid, x + 1, y - 1, &mut seen, &mut ret) }
    if !right            { inner(grid, x + 1, y,     &mut seen, &mut ret) }
    if !right && !bottom { inner(grid, x + 1, y + 1, &mut seen, &mut ret) }
    ret
}

fn part2(grid: &Grid) -> u32 {
    let mut sum = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if !matches!(grid.at(x, y), Cell::Sym('*')) { continue }
            let parts = touching_numbers(grid, x, y);
            if parts.len() == 2 {
                sum += parts[0] * parts[1];
            }
        }
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let grid = parse(&puzzle_input);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 467835);
    }
}
