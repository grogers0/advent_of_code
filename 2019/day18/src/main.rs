use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet, HashSet, VecDeque};
use std::io::{self, Read};

enum Square {
    Open,
    Key(String),
    Door(String),
    Wall
}

struct Map {
    width: usize,
    height: usize,
    entrances: Vec<(usize, usize)>,
    keys: BTreeSet<String>,
    data: Vec<Square>
}

impl Map {
    fn at(&self, x: usize, y: usize) -> &Square {
        assert!(x < self.width && y < self.height);
        &self.data[self.width * y + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Square {
        assert!(x < self.width && y < self.height);
        &mut self.data[self.width * y + x]
    }
}

fn parse(input: &str) -> Map {
    let input = input.trim();
    let width = input.lines().next().unwrap().chars().count();
    let mut entrance = None;
    let mut keys = BTreeSet::new();
    let mut data = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let sq = match ch {
                '.' => Square::Open,
                '#' => Square::Wall,
                '@' => {
                    assert!(entrance.is_none());
                    entrance = Some((x, y));
                    Square::Open
                },
                key if key.is_ascii_lowercase() => {
                    keys.insert(key.to_string());
                    Square::Key(key.to_string())
                },
                door if door.is_ascii_uppercase() =>
                    Square::Door(door.to_lowercase().to_string()),
                _ => panic!()
            };
            data.push(sq);
        }
    }
    assert!(data.len() % width == 0);
    let height = data.len() / width;
    Map {
        width: width,
        height: height,
        entrances: vec![entrance.unwrap()],
        keys: keys,
        data: data
    }
}

fn calc_next_moves(start_positions: Vec<(usize, usize)>,
    start_keys: &BTreeSet<String>, start_dist: usize, map: &Map,
    traverse_queue: &mut BinaryHeap<Reverse<(usize, Vec<(usize, usize)>, BTreeSet<String>)>>) {
    for i in 0..start_positions.len() {
        let mut fill_queue = VecDeque::new();
        fill_queue.push_back((start_positions[i], start_dist));
        let mut seen = HashSet::new();
        while let Some(((x, y), dist)) = fill_queue.pop_front() {
            if !seen.insert((x, y)) { continue }
            match map.at(x, y) {
                Square::Wall => continue,
                Square::Open => (),
                Square::Key(key) => if !start_keys.contains(key) {
                    let mut keys = start_keys.clone();
                    keys.insert(key.clone());
                    let mut positions = start_positions.clone();
                    positions[i] = (x, y);
                    traverse_queue.push(Reverse((dist, positions, keys)));
                    continue
                },
                Square::Door(key) => if !start_keys.contains(key) { continue }
            }
            if x > 0              { fill_queue.push_back(((x - 1, y), dist + 1)) }
            if x < map.width - 1  { fill_queue.push_back(((x + 1, y), dist + 1)) }
            if y > 0              { fill_queue.push_back(((x, y - 1), dist + 1)) }
            if y < map.height - 1 { fill_queue.push_back(((x, y + 1), dist + 1)) }
        }
    }
}

fn calc_best_path(map: &Map) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, map.entrances.clone(), BTreeSet::new())));
    let mut seen = HashSet::new();

    while let Some(Reverse((dist, positions, keys))) = queue.pop() {
        if !seen.insert((positions.clone(), keys.clone())) { continue }
        if keys == map.keys { return dist }
        calc_next_moves(positions, &keys, dist, &map, &mut queue);
    }
    panic!("failed to find a path to all keys")
}

fn modify_map_for_part2(map: &mut Map) {
    assert_eq!(map.entrances.len(), 1);
    let (x, y) = map.entrances[0];
    map.entrances = vec![
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1)
    ];
    *map.at_mut(x, y)     = Square::Wall;
    *map.at_mut(x - 1, y) = Square::Wall;
    *map.at_mut(x + 1, y) = Square::Wall;
    *map.at_mut(x, y - 1) = Square::Wall;
    *map.at_mut(x, y + 1) = Square::Wall;
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    calc_best_path(&map)
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    modify_map_for_part2(&mut map);
    calc_best_path(&map)
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
    fn test_part1_ex1() {
        let ex = "
#########
#b.A.@.a#
#########";
        assert_eq!(part1(ex), 8);
    }

    #[test]
    fn test_part1_ex2() {
        let ex = "
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(part1(ex), 86);
    }

    #[test]
    fn test_part1_ex3() {
        let ex = "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(part1(ex), 132);
    }

    #[test]
    fn test_part1_ex4() {
        let ex = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(part1(ex), 136);
    }

    #[test]
    fn test_part1_ex5() {
        let ex = "
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(part1(ex), 81);
    }


    #[test]
    fn test_part2_ex1() {
        let ex = "
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######";
        assert_eq!(part2(ex), 8);
    }

    #[test]
    fn test_part2_ex2() {
        let ex = "
###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############";
        assert_eq!(part2(ex), 24);
    }

    #[test]
    fn test_part2_ex3() {
        let ex = "
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############";
        assert_eq!(part2(ex), 32);
    }

    #[test]
    fn test_part2_ex4() {
        let ex = "
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
        assert_eq!(part2(ex), 72);
    }
}
