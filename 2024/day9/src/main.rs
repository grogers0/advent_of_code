use std::io::{self, Read};

#[derive(Clone)]
struct DiskSpan {
    begin: usize,
    count: usize,
}

struct DiskMap {
    files: Vec<DiskSpan>,
    free_list: Vec<DiskSpan>,
}

fn parse(puzzle_input: &str) -> DiskMap {
    let disk_map = puzzle_input.trim_end().chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
    let mut free_list = vec![];
    let mut files = vec![];

    let mut is_file = true;
    let mut idx = 0;
    for count in disk_map {
        let span = DiskSpan { begin: idx, count };
        if is_file {
            files.push(span);
        } else {
            free_list.push(span);
        }
        idx += count;
        is_file = !is_file;
    }
    DiskMap { files, free_list }
}

fn part1(disk_map: &DiskMap) -> usize {
    let max_index = {
        let span = disk_map.files.last().unwrap();
        span.begin + span.count - 1
    };
    let mut blocks: Vec<Option<usize>> = vec![None; max_index + 1];
    for (file_id, span) in disk_map.files.iter().enumerate() {
        for i in 0..span.count {
            blocks[span.begin + i] = Some(file_id);
        }
    }

    let mut free_idx = 0;
    let mut full_idx = blocks.len() - 1;
    while free_idx < full_idx {
        if blocks[free_idx].is_some() {
            free_idx += 1;
        } else if blocks[full_idx].is_none() {
            full_idx -= 1;
        } else {
            blocks[free_idx] = blocks[full_idx];
            blocks[full_idx] = None;
            free_idx += 1;
            full_idx -= 1;
        }
    }

    let mut sum = 0;
    for (i, block) in blocks.iter().enumerate() {
        if let Some(n) = block {
            sum += n * i as usize;
        }

    }
    sum
}

fn part2(disk_map: &DiskMap) -> usize {
    let mut files = disk_map.files.clone();
    let mut free_list = disk_map.free_list.clone();

    for file in files.iter_mut().rev() {
        let mut found_free_idx: Option<usize> = None;
        for (free_idx, free) in free_list.iter().enumerate() {
            if free.begin >= file.begin {
                break;
            } else if free.count >= file.count {
                found_free_idx = Some(free_idx);
                break;
            }
        }
        if let Some(free_idx) = found_free_idx {
            file.begin = free_list[free_idx].begin;
            free_list[free_idx].begin += file.count;
            free_list[free_idx].count -= file.count;
            if free_list[free_idx].count == 0 {
                free_list.remove(free_idx);
            }
        }
    }

    let mut sum = 0;
    for (file_id, file) in files.iter().enumerate() {
        for i in 0..file.count {
            sum += file_id * (file.begin + i);
        }
    }
    sum
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let disk_map = parse(&puzzle_input);
    println!("{}", part1(&disk_map));
    println!("{}", part2(&disk_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 2858);
    }
}
