use std::collections::HashMap;
use std::io::{self, Read};


struct DirEnt {
    is_dir: bool,
    bytes: u64,
    parent_idx: usize,
    children: HashMap<String, usize>, // path from cwd -> idx
}

impl DirEnt {
    fn new_dir(parent_idx: usize) -> Self {
        Self {
            is_dir: true,
            bytes: 0,
            parent_idx,
            children: HashMap::new(),
        }
    }
    fn new_file(parent_idx: usize, bytes: u64) -> Self {
        Self {
            is_dir: false,
            bytes,
            parent_idx,
            children: HashMap::new(),
        }
    }
}

struct Fs {
    entries: Vec<DirEnt>, // Root directory is idx 0
}

impl Fs {
    const ROOT: usize = 0;

    fn new() -> Self {
        Self {
            entries: vec![DirEnt::new_dir(0)],
        }
    }

    fn create_dir(&mut self, cwd: usize, path: &str) {
        if let None = self.entries[cwd].children.get(path) {
            let child = self.entries.len();
            self.entries.push(DirEnt::new_dir(cwd));
            self.entries[cwd].children.insert(path.to_string(), child);
        }
    }

    fn create_file(&mut self, mut cwd: usize, path: &str, bytes: u64) {
        assert!(self.entries[cwd].children.get(path).is_none());
        let child = self.entries.len();
        self.entries.push(DirEnt::new_file(cwd, bytes));
        self.entries[cwd].children.insert(path.to_string(), child);
        self.entries[cwd].bytes += bytes;
        while cwd != Fs::ROOT {
            cwd = self.entries[cwd].parent_idx;
            self.entries[cwd].bytes += bytes;
        }
    }
}

fn parse(puzzle_input: &str) -> Fs {
    let mut fs = Fs::new();
    let mut cwd = Fs::ROOT;
    const CD_CMD_PREFIX: &str = "$ cd ";
    const LS_CMD: &str = "$ ls";
    const LS_DIR_PREFIX: &str = "dir ";
    let mut ls_mode = false;

    for line in puzzle_input.trim_end().lines() {
        if line.starts_with(CD_CMD_PREFIX) {
            ls_mode = false;
            let path = &line[CD_CMD_PREFIX.len()..];
            match path {
                "/" => cwd = Fs::ROOT,
                ".." => cwd = fs.entries[cwd].parent_idx,
                _ => {
                    cwd = *fs.entries[cwd].children.get(path).unwrap();
                    assert!(fs.entries[cwd].is_dir);
                },
            }
        } else if line == LS_CMD {
            ls_mode = true;
        } else {
            assert!(ls_mode);
            if line.starts_with(LS_DIR_PREFIX) {
                let path = &line[LS_DIR_PREFIX.len()..];
                fs.create_dir(cwd, path);
            } else {
                let mut sp = line.split(" ");
                let bytes = sp.next().unwrap().parse().unwrap();
                let path = sp.next().unwrap();
                assert!(sp.next().is_none());
                fs.create_file(cwd, path, bytes);
            }
        }
    }
    fs
}

fn part1(fs: &Fs) -> u64 {
    fs.entries.iter()
        .filter(|ent| ent.is_dir && ent.bytes <= 100_000)
        .map(|ent| ent.bytes)
        .sum()
}

fn part2(fs: &Fs) -> u64 {
    let bytes_needed = 30_000_000 - (70_000_000 - fs.entries[Fs::ROOT].bytes);
    fs.entries.iter()
        .filter(|ent| ent.is_dir)
        .map(|ent| ent.bytes)
        .filter(|&bytes| bytes >= bytes_needed)
        .min().unwrap()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let fs = parse(&puzzle_input);
    println!("{}", part1(&fs));
    println!("{}", part2(&fs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EX)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EX)), 24933642);
    }
}
