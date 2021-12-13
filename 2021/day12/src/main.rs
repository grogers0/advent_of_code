use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{self, Read};

fn parse(puzzle_input: &str) -> (Vec<String>, Vec<Vec<usize>>) {
    let mut cave_names = BTreeSet::new();
    for line in puzzle_input.lines() {
        for name in line.split("-") {
            cave_names.insert(name.to_string());
        }
    }
    let cave_names: Vec<String> = cave_names.into_iter().collect();
    let mut adj_list = vec![Vec::new(); cave_names.len()];
    for line in puzzle_input.lines() {
        let mut split_iter = line.split("-");
        let name1 = split_iter.next().unwrap().to_string();
        let name2 = split_iter.next().unwrap().to_string();
        assert!(split_iter.next().is_none());
        let name1_idx = cave_names.binary_search(&name1).unwrap();
        let name2_idx = cave_names.binary_search(&name2).unwrap();
        adj_list[name1_idx].push(name2_idx);
        adj_list[name2_idx].push(name1_idx);
    }
    (cave_names, adj_list)
}

fn part1(cave_names: &[String], adj_list: &[Vec<usize>]) -> usize {
    let start_room_id = cave_names.binary_search(&"start".to_string()).unwrap();
    let end_room_id = cave_names.binary_search(&"end".to_string()).unwrap();
    let small_rooms: HashSet<usize> = cave_names.iter().enumerate()
        .filter_map(|(i, name)| if name.chars().next().unwrap().is_lowercase() { Some(i) } else { None })
        .collect();

    let mut all_paths = HashSet::new();
    let mut queue = VecDeque::new();
    let start_path = vec![start_room_id];
    let start_path_rooms = start_path.iter().cloned().collect::<HashSet<_>>();
    queue.push_back((start_path, start_path_rooms));
    while let Some((path, path_rooms)) = queue.pop_front() {
        let curr_room_id = *path.last().unwrap();
        if curr_room_id == end_room_id {
            all_paths.insert(path);
            continue;
        }
        for &next_room_id in adj_list[curr_room_id].iter() {
            if !small_rooms.contains(&next_room_id) || !path_rooms.contains(&next_room_id) {
                let mut next_path = path.clone();
                let mut next_path_rooms = path_rooms.clone();
                next_path.push(next_room_id);
                next_path_rooms.insert(next_room_id);
                queue.push_back((next_path, next_path_rooms));
            }
        }
    }
    all_paths.len()
}

fn part2(cave_names: &[String], adj_list: &[Vec<usize>]) -> usize {
    let start_room_id = cave_names.binary_search(&"start".to_string()).unwrap();
    let end_room_id = cave_names.binary_search(&"end".to_string()).unwrap();
    let small_rooms: HashSet<usize> = cave_names.iter().enumerate()
        .filter_map(|(i, name)| if name.chars().next().unwrap().is_lowercase() { Some(i) } else { None })
        .collect();

    let mut all_paths = HashSet::new();
    let mut queue = VecDeque::new();
    let start_path = vec![start_room_id];
    let start_path_rooms = start_path.iter().cloned().collect::<HashSet<_>>();
    queue.push_back((start_path, start_path_rooms, false));
    while let Some((path, path_rooms, small_revisited)) = queue.pop_front() {
        let curr_room_id = *path.last().unwrap();
        if curr_room_id == end_room_id {
            all_paths.insert(path);
            continue;
        }
        for &next_room_id in adj_list[curr_room_id].iter() {
            if next_room_id == start_room_id { continue }
            if !small_rooms.contains(&next_room_id) || !path_rooms.contains(&next_room_id) || !small_revisited {
                let mut next_path = path.clone();
                let mut next_path_rooms = path_rooms.clone();
                next_path.push(next_room_id);
                next_path_rooms.insert(next_room_id);
                let small_revisited = small_revisited || (small_rooms.contains(&next_room_id) && path_rooms.contains(&next_room_id));
                queue.push_back((next_path, next_path_rooms, small_revisited));
            }
        }
    }
    all_paths.len()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (cave_names, adj_list) = parse(&puzzle_input);
    println!("{}", part1(&cave_names, &adj_list));
    println!("{}", part2(&cave_names, &adj_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EX2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EX3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() {
        let (cave_names, adj_list) = parse(EX1);
        assert_eq!(10, part1(&cave_names, &adj_list));
        let (cave_names, adj_list) = parse(EX2);
        assert_eq!(19, part1(&cave_names, &adj_list));
        let (cave_names, adj_list) = parse(EX3);
        assert_eq!(226, part1(&cave_names, &adj_list));
    }

    #[test]
    fn test_part2() {
        let (cave_names, adj_list) = parse(EX1);
        assert_eq!(36, part2(&cave_names, &adj_list));
        let (cave_names, adj_list) = parse(EX2);
        assert_eq!(103, part2(&cave_names, &adj_list));
        let (cave_names, adj_list) = parse(EX3);
        assert_eq!(3509, part2(&cave_names, &adj_list));
    }
}
