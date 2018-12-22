use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::io::{self, Read};
use std::iter::Peekable;


#[derive(Copy, Clone, Debug)]
enum Dir {
    North, South, East, West
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::North => write!(f, "N"),
            Dir::South => write!(f, "S"),
            Dir::East  => write!(f, "E"),
            Dir::West  => write!(f, "W")
        }
    }
}

#[derive(Clone, Debug)]
enum Regex {
    Dir(Dir),
    Seq(Vec<Box<Regex>>),
    Branch(Vec<Box<Regex>>)
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_internal(regex: &Regex, f: &mut fmt::Formatter) -> fmt::Result {
            match regex {
                Regex::Dir(dir) => write!(f, "{}", dir)?,
                Regex::Seq(seq) => {
                    for r in seq {
                        fmt_internal(r, f)?;
                    }
                },
                Regex::Branch(branch) => {
                    write!(f, "(")?;
                    let mut first = true;
                    for r in branch {
                        if first {
                            first = false;
                        } else {
                            write!(f, "|")?;
                        }
                        fmt_internal(r, f)?;
                    }
                    write!(f, ")")?;
                }

            }
            Ok(())
        }

        write!(f, "^")?;
        fmt_internal(self, f)?;
        write!(f, "$")
    }
}

impl Regex {
    fn parse_branch(chars: &mut Peekable<impl Iterator<Item=char>>) -> Regex {
        let mut branch = Vec::new();
        loop {
            branch.push(Box::new(Regex::parse_seq(chars)));
            match chars.peek() {
                None => unreachable!(),
                Some('|') => { chars.next(); },
                Some(')') => { chars.next(); break; },
                Some(_) => unreachable!()
            }
        }
        Regex::Branch(branch)
    }

    fn parse_seq(chars: &mut Peekable<impl Iterator<Item=char>>) -> Regex {
        let mut ret = Vec::new();
        while let Some(ch) = chars.peek() {
            if *ch == ')' || *ch == '|' { break }
            let ch = chars.next().unwrap();
            match ch {
                '^' | '$' => (), // ignore
                '(' => ret.push(Box::new(Regex::parse_branch(chars))),
                'N' => ret.push(Box::new(Regex::Dir(Dir::North))),
                'S' => ret.push(Box::new(Regex::Dir(Dir::South))),
                'E' => ret.push(Box::new(Regex::Dir(Dir::East))),
                'W' => ret.push(Box::new(Regex::Dir(Dir::West))),
                _ => unreachable!()
            }
        }
        Regex::Seq(ret)
    }

    fn parse(input: &str) -> Regex {
        Regex::parse_seq(&mut input.trim_end().chars().peekable())
    }
}

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<char>>,
    offset: (usize, usize)
}

impl Map {
    fn generate(regex: &Regex) -> Map {
        let mut map = Map::fresh();
        let mut pos = (1, 1);

        map.fill(&mut pos, regex);
        assert_eq!(map.map[map.offset.1][map.offset.0], 'X');
        map.seal();
        map
    }

    fn fill(&mut self, pos: &mut (usize, usize), regex: &Regex) {
        match regex {
            Regex::Dir(dir) => self.fill_dir(pos, *dir),
            Regex::Seq(seq) => self.fill_seq(pos, seq),
            Regex::Branch(branch) => self.fill_branch(pos, branch)
        }
    }

    fn fill_dir(&mut self, pos: &mut (usize, usize), dir: Dir) {
        match dir {
            Dir::North => {
                self.push_row_north_if_needed(pos);
                self.map[pos.1-1][pos.0] = '-';
                pos.1 -= 2;
            },
            Dir::South => {
                self.push_row_south_if_needed(pos);
                self.map[pos.1+1][pos.0] = '-';
                pos.1 += 2;
            },
            Dir::East => {
                self.push_col_east_if_needed(pos);
                self.map[pos.1][pos.0+1] = '|';
                pos.0 += 2;
            },
            Dir::West => {
                self.push_col_west_if_needed(pos);
                self.map[pos.1][pos.0-1] = '|';
                pos.0 -= 2;
            }
        }
    }

    fn fill_seq(&mut self, pos: &mut (usize, usize), seq: &Vec<Box<Regex>>) {
        for regex in seq.iter() {
            self.fill(pos, regex);
        }
    }

    fn fill_branch(&mut self, pos: &mut (usize, usize), branch: &Vec<Box<Regex>>) {
        let init_offset = self.offset;
        let init_pos = *pos;
        for detour in branch.iter() {
            *pos = init_pos;
            pos.0 += self.offset.0 - init_offset.0;
            pos.1 += self.offset.1 - init_offset.1;
            self.fill(pos, &**detour);
        }
    }

    fn room_row(len: usize) -> Vec<char> {
        let mut row = Vec::with_capacity(len);
        for _ in 0 .. len/2 {
            row.push('?');
            row.push('.');
        }
        row.push('?');
        row
    }

    fn wall_row(len: usize) -> Vec<char> {
        let mut row = Vec::with_capacity(len);
        for _ in 0 .. len/2 {
            row.push('#');
            row.push('?');
        }
        row.push('#');
        row
    }

    fn push_row_north_if_needed(&mut self, pos: &mut (usize, usize)) {
        if pos.1 > 1 { return }
        let width = self.map[0].len();
        self.map.insert(0, Self::room_row(width));
        self.map.insert(0, Self::wall_row(width));
        pos.1 += 2;
        self.offset.1 += 2;
    }

    fn push_row_south_if_needed(&mut self, pos: &(usize, usize)) {
        if pos.1 < self.map.len() - 2 { return }
        let width = self.map[0].len();
        self.map.push(Self::room_row(width));
        self.map.push(Self::wall_row(width));
    }

    fn push_col_west_if_needed(&mut self, pos: &mut (usize, usize)) {
        if pos.0 > 1 { return }
        let height = self.map.len();
        for i in 0 .. height/2 {
            self.map[i*2].insert(0, '?');
            self.map[i*2+1].insert(0, '.');
            self.map[i*2].insert(0, '#');
            self.map[i*2+1].insert(0, '?');
        }
        self.map[height-1].insert(0, '?');
        self.map[height-1].insert(0, '#');
        pos.0 += 2;
        self.offset.0 += 2;
    }

    fn push_col_east_if_needed(&mut self, pos: &(usize, usize)) {
        if pos.0 < self.map[0].len() - 2 { return }
        let height = self.map.len();
        for i in 0 .. height/2 {
            self.map[i*2].push('?');
            self.map[i*2+1].push('.');
            self.map[i*2].push('#');
            self.map[i*2+1].push('?');
        }
        self.map[height-1].push('?');
        self.map[height-1].push('#');
    }

    fn seal(&mut self) {
        for row in self.map.iter_mut() {
            for ch in row.iter_mut() {
                if *ch == '?' { *ch = '#' }
            }
        }
    }

    fn fresh() -> Map {
        Map {
            map: vec![vec!['#', '?', '#'],
                      vec!['?', 'X', '?'],
                      vec!['#', '?', '#']],
            offset: (1, 1)
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.map.iter().enumerate() {
            for ch in row.iter() {
                write!(f, "{}", ch)?;
            }
            if i != self.map.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn calc_room_distances(map: &Map) -> BTreeMap<(usize, usize), usize> {
    let mut seen_dists = BTreeMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((map.offset, 0));
    while let Some(((x, y), dist)) = queue.pop_front() {
        if seen_dists.contains_key(&(x, y)) { continue }
        seen_dists.insert((x, y), dist);
        if x > 1                    && map.map[y][x-1] == '|' { queue.push_back(((x-2, y), dist+1)); }
        if x < map.map[0].len() - 2 && map.map[y][x+1] == '|' { queue.push_back(((x+2, y), dist+1)); }
        if y > 1                    && map.map[y-1][x] == '-' { queue.push_back(((x, y-2), dist+1)); }
        if y < map.map.len() - 2    && map.map[y+1][x] == '-' { queue.push_back(((x, y+2), dist+1)); }
    }
    seen_dists
}

fn part1(input: &str) -> usize {
    let map = Map::generate(&Regex::parse(input));
    let room_dists = calc_room_distances(&map);
    *room_dists.values().max().unwrap()
}

fn part2(input: &str) -> usize {
    let map = Map::generate(&Regex::parse(input));
    let room_dists = calc_room_distances(&map);
    room_dists.iter().filter(|(_,dist)| **dist >= 1000).count()
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
    fn test_map() {
        assert_eq!(&format!("{}", Map::generate(&Regex::parse("^WNE$"))), "\
#####
#.|.#
#-###
#.|X#
#####");
        assert_eq!(&format!("{}", Map::generate(&Regex::parse("^ENWWW(NEEE|SSE(EE|N))$"))), "\
#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########");
        assert_eq!(&format!("{}", Map::generate(&Regex::parse("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"))), "\
###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########");
        assert_eq!(&format!("{}", Map::generate(&Regex::parse("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"))), "\
#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############");
        assert_eq!(&format!("{}", Map::generate(&Regex::parse("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"))), "\
###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("^WNE$"), 3);
        assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
        assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
        assert_eq!(part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), 23);
        assert_eq!(part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), 31);

        assert_eq!(part1("^EEE(S|N)EEEEEEEEEEESSWWWWWWWWWWWN$"), 16);
    }
}
