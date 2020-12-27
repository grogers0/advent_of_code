use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

#[derive(Clone)]
struct Tile {
    sidelen: usize,
    pixels: Vec<bool>
}

impl Tile {
    fn top(&self) -> Vec<bool> {
        (0..self.sidelen).map(|x| self.pixels[x]).collect()
    }

    fn bottom(&self) -> Vec<bool> {
        (0..self.sidelen).map(|x| self.pixels[(self.sidelen-1)*self.sidelen + x]).collect()
    }

    fn left(&self) -> Vec<bool> {
        (0..self.sidelen).map(|y| self.pixels[y*self.sidelen]).collect()
    }

    fn right(&self) -> Vec<bool> {
        (0..self.sidelen).map(|y| self.pixels[y*self.sidelen + self.sidelen - 1]).collect()
    }

    fn flip(&self) -> Tile {
        let mut pixels = vec![false; self.sidelen * self.sidelen];
        for y in 0..self.sidelen {
            for x in 0..self.sidelen {
                pixels[(self.sidelen - y - 1)*self.sidelen + x] =
                    self.pixels[y*self.sidelen + x];
            }
        }
        Tile { sidelen: self.sidelen, pixels }
    }

    fn rotate(&self) -> Tile {
        let mut pixels = vec![false; self.sidelen * self.sidelen];
        for y in 0..self.sidelen {
            for x in 0..self.sidelen {
                pixels[x*self.sidelen + self.sidelen - y - 1] =
                    self.pixels[y*self.sidelen + x];
            }
        }
        Tile { sidelen: self.sidelen, pixels }
    }

    fn all_rotations(&self) -> Vec<Tile> {
        let mut ret: Vec<Tile> = Vec::new();
        let mut normal = self.clone();
        let mut flipped = self.flip();
        for _ in 0..4 {
            ret.push(normal.clone());
            ret.push(flipped.clone());
            normal = normal.rotate();
            flipped = flipped.rotate();
        }
        debug_assert_eq!(8, ret.len());
        ret
    }

}

fn flip_edge(edge: &Vec<bool>) -> Vec<bool> {
    let mut flipped = Vec::with_capacity(edge.len());
    for &px in edge.iter().rev() {
        flipped.push(px);
    }
    flipped
}

fn parse(puzzle_input: &str) -> HashMap<u16, Tile> {
    fn parse_tile(s: &str) -> (u16, Tile) {
        let mut id = 0;
        let sidelen = s.lines().count() - 1;
        debug_assert_eq!(sidelen, s.lines().skip(1).next().unwrap().chars().count());
        let mut pixels = Vec::with_capacity(sidelen * sidelen);
        fn ch_to_pixel(ch: char) -> bool {
            match ch {
                '.' => false,
                '#' => true,
                _ => panic!()
            }
        }

        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                debug_assert!(line.starts_with("Tile ") && line.ends_with(":"));
                id = line[5..(line.len()-1)].parse().unwrap();
            } else {
                debug_assert_eq!(sidelen, line.chars().count());
                for pixel in line.chars().map(ch_to_pixel) {
                    pixels.push(pixel);
                }
            }
        }
        debug_assert!(id != 0);
        debug_assert_eq!(sidelen*sidelen, pixels.len());
        (id, Tile { sidelen, pixels })
    }

    puzzle_input.trim_end().split("\n\n").map(parse_tile).collect()
}

fn isqrt(x: usize) -> usize {
    for i in 0..=x/2 {
        if i*i == x { return i }
    }
    panic!()
}

fn matching_tile(edge: &Vec<bool>, edge_map: &HashMap<Vec<bool>, HashSet<u16>>, used: &HashSet<u16>) -> Option<u16> {
    let ids = edge_map.get(edge).unwrap();
    ids.iter().filter(|&id| !used.contains(id)).cloned().next()
}

fn fill_image(top_left_id: u16, top_left_tile: Tile, tiles: &HashMap<u16, Tile>, edge_map: &HashMap<Vec<bool>, HashSet<u16>>) -> Option<(Vec<u16>, Vec<Tile>)> {
    let mut used = HashSet::new();
    used.insert(top_left_id);
    let squarelen = isqrt(tiles.len());
    let mut image_tiles = Vec::with_capacity(tiles.len());
    image_tiles.push(top_left_tile);
    let mut image_ids = Vec::with_capacity(tiles.len());
    image_ids.push(top_left_id);

    for y in 0..squarelen {
        'x: for x in 0..squarelen {
            if y == 0 && x == 0 { continue }
            let left_idx = if x > 0 { Some(y*squarelen + x - 1) } else { None };
            let above_idx = if y > 0 { Some((y-1)*squarelen + x) } else { None };
            let id = if let Some(left_idx) = left_idx {
                matching_tile(&image_tiles[left_idx].right(), edge_map, &used)?
            } else if let Some(above_idx) = above_idx {
                matching_tile(&image_tiles[above_idx].bottom(), edge_map, &used)?
            } else {
                panic!();
            };

            for tile in tiles.get(&id).unwrap().all_rotations() {
                if left_idx.map_or(true, |idx| image_tiles[idx].right() == tile.left()) &&
                    above_idx.map_or(true, |idx| image_tiles[idx].bottom() == tile.top())
                {
                    image_tiles.push(tile);
                    image_ids.push(id);
                    used.insert(id);
                    continue 'x;
                }
            }
            return None;
        }
    }
    Some((image_ids, image_tiles))
}

fn create_image(tiles: &HashMap<u16, Tile>) -> (Vec<u16>, Vec<Tile>) {
    let mut edge_map = HashMap::<Vec<bool>, HashSet<u16>>::new();
    for (id, tile) in tiles {
        for edge in vec![tile.top(), tile.bottom(), tile.left(), tile.right()] {
            for edge in vec![edge.clone(), flip_edge(&edge)] {
                if let Some(edges) = edge_map.get_mut(&edge) {
                    edges.insert(*id);
                } else {
                    let mut edges = HashSet::new();
                    edges.insert(*id);
                    edge_map.insert(edge.clone(), edges);
                }
            }
        }
    }

    for (id, tile) in tiles {
        for tile in tile.all_rotations() {
            if let Some(ret) = fill_image(*id, tile, &tiles, &edge_map) {
                return ret;
            }
        }
    }
    panic!()
}

fn part1(puzzle_input: &str) -> u64 {
    let tiles = parse(puzzle_input);
    let squarelen = isqrt(tiles.len());
    let (ids, _) = create_image(&tiles);
    let id1 = ids[0] as u64;
    let id2 = ids[squarelen - 1] as u64;
    let id3 = ids[tiles.len() - 1] as u64;
    let id4 = ids[tiles.len() - squarelen] as u64;
    return id1 * id2 * id3 * id4;
}

fn remove_borders(tiles: Vec<Tile>) -> Tile {
    let mut pixels = Vec::new();
    let squarelen = isqrt(tiles.len());
    let sidelen = tiles[0].sidelen;

    for ytile in 0..squarelen {
        for y in 1..sidelen-1 {
            for xtile in 0..squarelen {
                for x in 1..sidelen-1 {
                    pixels.push(tiles[ytile*squarelen + xtile].pixels[y*sidelen + x]);
                }
            }
        }
    }
    Tile { sidelen: isqrt(pixels.len()), pixels }
}

fn seamonster_pixels() -> Vec<Vec<bool>> {
    vec![
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,  true, false],
        vec![ true, false, false, false, false,  true,  true, false, false, false, false,  true,  true, false, false, false, false,  true,  true,  true],
        vec![false,  true, false, false,  true, false, false,  true, false, false,  true, false, false,  true, false, false,  true, false, false, false]
    ]
}

fn is_seamonster(tile: &Tile, x: usize, y: usize, sm_pixels: &Vec<Vec<bool>>) -> bool {
    for (j, sm_row) in sm_pixels.iter().enumerate() {
        for (i, sm_px) in sm_row.iter().enumerate() {
            if *sm_px && !tile.pixels[(y+j)*tile.sidelen + x+i] {
                return false
            }
        }
    }
    true
}

fn remove_seamonster(tile: &mut Tile, x: usize, y: usize, sm_pixels: &Vec<Vec<bool>>) {
    for (j, sm_row) in sm_pixels.iter().enumerate() {
        for (i, sm_px) in sm_row.iter().enumerate() {
            if *sm_px {
                tile.pixels[(y+j)*tile.sidelen + x+i] = false;
            }
        }
    }
}

fn mask_seamonsters(tile: &mut Tile) -> bool {
    let sm_pixels = seamonster_pixels();
    let sm_height = sm_pixels.len();
    let sm_width = sm_pixels[0].len();
    let mut any_sms = false;
    for y in 0..tile.sidelen-sm_height {
        for x in 0..tile.sidelen-sm_width {
            if is_seamonster(tile, x, y, &sm_pixels) {
                any_sms = true;
                remove_seamonster(tile, x, y, &sm_pixels);
            }
        }
    }
    any_sms
}

fn part2(puzzle_input: &str) -> usize {
    let tiles = parse(puzzle_input);
    let (_, tiles) = create_image(&tiles);
    let tile = remove_borders(tiles);
    for mut tile in tile.all_rotations() {
        if mask_seamonsters(&mut tile) {
            return tile.pixels.iter().filter(|&&px| px).count();
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_part1() {
        assert_eq!(20899048083289, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(273, part2(EX));
    }
}
