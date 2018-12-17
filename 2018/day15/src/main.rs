use std::collections::{BTreeSet, VecDeque};
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UnitType {
    Elf,
    Goblin
}

#[derive(Clone, Debug)]
struct Unit {
    unit_type: UnitType,
    position: (usize, usize),
    hp: usize,
    attack: usize
}

impl Unit {
    fn new(unit_type: UnitType, x: usize, y: usize) -> Unit {
        Unit {
            unit_type: unit_type,
            position: (x, y),
            hp: 200,
            attack: 3
        }
    }
}

// Returns (walls, units)
fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<Unit>) {
    let mut units = Vec::new();
    let walls = input.lines().enumerate().map(|(y,line)| {
        line.chars().enumerate().map(|(x,ch)| {
            match ch {
                '#' => true,
                '.' => false,
                'G' => {
                    units.push(Unit::new(UnitType::Goblin, x, y));
                    false
                },
                'E' => {
                    units.push(Unit::new(UnitType::Elf, x, y));
                    false
                },
                _ => unreachable!()
            }
        })
        .collect()
    })
    .collect();

    (walls, units)
}

#[allow(dead_code)]
fn print_board(walls: &Vec<Vec<bool>>, units: &Vec<Unit>) -> String {
    let mut out = String::new();
    for (y,row) in walls.iter().enumerate() {
        for (x,wall) in row.iter().enumerate() {
            let ch = if *wall {
                '#'
            } else if let Some(unit) = units.iter().find(|unit| unit.position == (x,y)) {
                match unit.unit_type {
                    UnitType::Elf => 'E',
                    UnitType::Goblin => 'G'
                }
            } else {
                '.'
            };
            out.push(ch);
        }
        out.push('\n');
    }
    out
}

fn enemy_unit_type(unit_type: UnitType) -> UnitType {
    match unit_type {
        UnitType::Elf => UnitType::Goblin,
        UnitType::Goblin => UnitType::Elf
    }
}

fn get_enemy_targets(unit_type: UnitType, units: &Vec<Unit>) -> Vec<(usize, usize)> {
    let enemy_type = enemy_unit_type(unit_type);
    units.iter()
        .filter(|unit| unit.unit_type == enemy_type)
        .map(|unit| unit.position)
        .collect()
}

fn unoccupied((x, y): (usize, usize), walls: &Vec<Vec<bool>>, unit_positions: &BTreeSet<(usize, usize)>) -> bool {
    !walls[y][x] && !unit_positions.contains(&(x, y))
}

fn get_unoccupied_adjacent_squares(targets: Vec<(usize, usize)>, walls: &Vec<Vec<bool>>,
                                   unit_positions: &BTreeSet<(usize, usize)>) -> Vec<(usize, usize)> {
    targets.into_iter().map(|pos| adjacent(pos))
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|positions| positions)
        .filter(|pos| unoccupied(**pos, walls, unit_positions))
        .cloned()
        .collect()
}

fn adjacent((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

fn get_distance(from: (usize, usize), to: (usize, usize), walls: &Vec<Vec<bool>>,
                unit_positions: &BTreeSet<(usize, usize)>) -> Option<usize> {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((from, 0));
    while let Some(((x,y), dist)) = queue.pop_front() {
        if seen.contains(&(x,y)) || from != (x,y) && !unoccupied((x,y), walls, unit_positions) {
            continue;
        } else if (x,y) == to {
            return Some(dist);
        }
        seen.insert((x,y));

        for pos in adjacent((x, y)).into_iter() {
            queue.push_back((*pos, dist + 1));
        }
    }
    None
}

fn choose_nearest_reachable_square(from: (usize, usize), to_squares: Vec<(usize, usize)>,
                                   walls: &Vec<Vec<bool>>, unit_positions: &BTreeSet<(usize, usize)>) -> Option<(usize, usize)> {
    let mut nearest = to_squares.into_iter()
        .map(|pos| (pos, get_distance(from, pos, walls, unit_positions)))
        .filter(|(_, dist_opt)| dist_opt.is_some())
        .map(|(pos, dist_opt)| (pos, dist_opt.unwrap()))
        .collect::<Vec<((usize,usize),usize)>>();
    nearest.sort_by_key(|((x,y),dist)| (*dist, *y, *x));
    if nearest.is_empty() {
        None
    } else {
        Some(nearest[0].0)
    }
}

fn move_to(unit: &Unit, units: &Vec<Unit>, walls: &Vec<Vec<bool>>) -> (usize, usize) {
    let unit_positions = units.iter().map(|unit| unit.position).collect();
    let to_squares = get_enemy_targets(unit.unit_type, units);
    if to_squares.iter().any(|to| adjacent(*to).contains(&unit.position)) {
        return unit.position;
    }
    let to_squares = get_unoccupied_adjacent_squares(to_squares, walls, &unit_positions);
    if let Some((dest_x, dest_y)) = choose_nearest_reachable_square(unit.position, to_squares, walls, &unit_positions) {
        let mut next_squares = adjacent(unit.position).into_iter()
            .filter(|pos| unoccupied(**pos, walls, &unit_positions))
            .map(|pos| (*pos, get_distance(*pos, (dest_x, dest_y), walls, &unit_positions).unwrap_or(std::usize::MAX)))
            .collect::<Vec<((usize, usize), usize)>>();
        next_squares.sort_by_key(|((x, y), dist)| (*dist, *y, *x));
        return next_squares[0].0;
    }
    unit.position
}

// Returns the index of the target it killed (if any)
fn attack(unit_idx: usize, units: &mut Vec<Unit>) -> Option<usize> {
    let power = units[unit_idx].attack;
    let enemy_type = enemy_unit_type(units[unit_idx].unit_type);
    let mut candidates = adjacent(units[unit_idx].position).into_iter()
        .map(|pos| (0..units.len()).find(|i| units[*i].position == *pos && units[*i].unit_type == enemy_type))
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .collect::<Vec<_>>();
    candidates.sort_by_key(|i| (units[*i].hp, units[*i].position.1, units[*i].position.0));
    if !candidates.is_empty() {
        if units[candidates[0]].hp <= power {
            units.remove(candidates[0]);
            return Some(candidates[0]);
        } else {
            units[candidates[0]].hp -= power;
        }
    }
    None
}

fn targets_left(units: &Vec<Unit>) -> bool {
    units.iter().any(|unit| unit.unit_type == UnitType::Elf) && 
        units.iter().any(|unit| unit.unit_type == UnitType::Goblin)
}

// Returns true if it was a full round
fn execute_round(walls: &Vec<Vec<bool>>, units: &mut Vec<Unit>) -> bool {
    units.sort_by_key(|unit| (unit.position.1, unit.position.0));
    let mut i = 0;
    while targets_left(units) && i < units.len() {
        units[i].position = move_to(&units[i], units, walls);
        if let Some(killed) = attack(i, units) {
            if killed > i { i += 1; }
        } else  {
            i += 1;
        }
    }
    i == units.len()
}

fn execute_rounds(walls: &Vec<Vec<bool>>, units: &mut Vec<Unit>) -> usize {
    let mut rounds = 0;
    loop {
        if execute_round(walls, units) {
            rounds += 1;
        } else {
            break
        }
    }
    rounds
}

fn count_elves(units: &Vec<Unit>) -> usize {
    units.iter().filter(|unit| unit.unit_type == UnitType::Elf).count()
}

fn part1(input: &str) -> usize {
    let (walls, mut units) = parse(&input);
    let rounds = execute_rounds(&walls, &mut units);
    rounds * units.iter().map(|unit| unit.hp).sum::<usize>()
}

fn part2(input: &str) -> usize {
    let (walls, starting_units) = parse(&input);
    let num_starting_elves = count_elves(&starting_units);
    for attack in 4.. {
        let mut units = starting_units.clone();
        for unit in units.iter_mut() {
            if unit.unit_type == UnitType::Elf { unit.attack = attack }
        }
        let rounds = execute_rounds(&walls, &mut units);
        if num_starting_elves == count_elves(&units) {
            return rounds * units.iter().map(|unit| unit.hp).sum::<usize>();
        }
    }
    unreachable!()
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
    fn test_move() {
        let ex = "\
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
        let result = "\
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########";
        let (walls, mut units) = parse(&ex);
        for _ in 0..3 {
            execute_round(&walls, &mut units);
        }
        assert_eq!(print_board(&walls, &units).trim_end(), result);
        execute_round(&walls, &mut units);
        assert_eq!(print_board(&walls, &units).trim_end(), result);
    }

    const EX1: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const EX2: &str = "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const EX3: &str = "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const EX4: &str = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const EX5: &str = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const EX6: &str = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 27730);
        assert_eq!(part1(EX2), 36334);
        assert_eq!(part1(EX3), 39514);
        assert_eq!(part1(EX4), 27755);
        assert_eq!(part1(EX5), 28944);
        assert_eq!(part1(EX6), 18740);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EX1), 4988);
        assert_eq!(part2(EX3), 31284);
        assert_eq!(part2(EX4), 3478);
        assert_eq!(part2(EX5), 6474);
        assert_eq!(part2(EX6), 1140);
    }

}
