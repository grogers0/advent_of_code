use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref GLYPHS: HashMap<&'static str, char> = {
        let mut map = HashMap::new();
        map.insert("
.##..
#..#.
####.
#..#.
#..#.
#..#.".trim_start(), 'A');
        map.insert("
###..
#..#.
###..
#..#.
#..#.
###..".trim_start(), 'B');
        map.insert("
.###.
#....
#....
#....
#....
.###.".trim_start(), 'C');
        map.insert("
.##..
#..#.
#....
#....
#..#.
.##..".trim_start(), 'C');
        map.insert("
###..
#..#.
#..#.
#..#.
#..#.
###..".trim_start(), 'D');
        map.insert("
####.
#....
###..
#....
#....
####.".trim_start(), 'E');
        map.insert("
####.
#....
###..
#....
#....
#....".trim_start(), 'F');
        map.insert("
.##..
#..#.
#....
#.##.
#..#.
.###.".trim_start(), 'G');
        map.insert("
#..#.
#..#.
####.
#..#.
#..#.
#..#.".trim_start(), 'H');
        map.insert("
#....
#....
#....
#....
#....
####.".trim_start(), 'L');
        map.insert("
.##..
#..#.
#..#.
#..#.
#..#.
.##..".trim_start(), 'O');
        map.insert("
###..
#..#.
#..#.
###..
#....
#....".trim_start(), 'P');
        map.insert("
###..
#..#.
#..#.
###..
#..#.
#..#.".trim_start(), 'R');
        map.insert("
###..
#..#.
#..#.
###..
#.#..
#..#.".trim_start(), 'R');
        map.insert("
#..#.
#..#.
#..#.
#..#.
#..#.
.##..".trim_start(), 'U');
        map.insert("
####.
...#.
..#..
.#...
#....
####.".trim_start(), 'Z');
        map
    };
}

pub fn decode(pixels: &str) -> Result<String, String> {
    let height = pixels.lines().count();
    if height == 0 { return Err("Height must be nonzero".to_string()) }
    if height % 6 != 0 { return Err(format!("Height must be a multiple of 6 but was {}", height)) }

    let width = pixels.lines().next().unwrap().chars().count();
    if width == 0 { return Err("Width must be nonzero".to_string()) }
    if width % 5 != 0 { return Err(format!("Width must be a multiple of 5 but was {}", width)) }

    let mut glyphs = vec![String::new(); width/5];
    for (y, line) in pixels.lines().enumerate() {
        if line.chars().count() != width { return Err("Image must be rectangular".to_string()) }
        for (x, ch) in line.chars().enumerate() {
            let glyph_num = x / 5;
            if y != 0 && x % 5 == 0 { glyphs[glyph_num].push('\n') }
            glyphs[glyph_num].push(ch);
        }
    }

    let mut ret = String::new();
    for glyph in glyphs {
        if let Some(&ch) = GLYPHS.get(glyph.as_str()) {
            ret.push(ch);
        } else {
            return Err(format!("Failed to parse glyph:\n{}", glyph))
        }
    }

    Ok(ret)
}
