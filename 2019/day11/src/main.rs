use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::mpsc::channel;
use std::thread;

use intcode::*;

fn rotate(dir: (i64, i64), turn: i64) -> (i64, i64) {
    match turn {
        0 => (dir.1, -dir.0), // turn left
        1 => (-dir.1, dir.0), // turn right
        _ => panic!()
    }
}

fn panel_color(panels: &HashMap<(i64, i64), i64>, pos: (i64, i64)) -> i64 {
    *panels.get(&pos).unwrap_or(&0)
}

fn paint_panels(mem_str: &str, panels: &mut HashMap<(i64, i64), i64>) {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    let mut mem = parse(mem_str);
    thread::spawn(move || run(&mut mem, rx_in, tx_out));

    let mut pos = (0, 0);
    let mut dir = (0, -1);
    tx_in.send(panel_color(panels, pos)).unwrap();
    while let Ok(color) = rx_out.recv() {
        let turn = rx_out.recv().unwrap();

        panels.insert(pos, color);
        dir = rotate(dir, turn);
        pos.0 += dir.0;
        pos.1 += dir.1;

        // OK if the program has stopped already and closed the channel
        let _ = tx_in.send(panel_color(panels, pos));
    }
}

fn part1(mem_str: &str) -> usize {
    let mut panels = HashMap::new();
    paint_panels(mem_str, &mut panels);
    panels.len()
}

// TODO - parse the pixel values into letters, also in day8
fn part2(mem_str: &str) -> String {
    let mut panels = HashMap::new();
    panels.insert((0,0), 1);
    paint_panels(mem_str, &mut panels);

    let xmin = panels.iter().filter(|(_,c)| **c == 1).map(|((x,_),_)| *x).min().unwrap();
    let xmax = panels.iter().filter(|(_,c)| **c == 1).map(|((x,_),_)| *x).max().unwrap();
    let ymin = panels.iter().filter(|(_,c)| **c == 1).map(|((_,y),_)| *y).min().unwrap();
    let ymax = panels.iter().filter(|(_,c)| **c == 1).map(|((_,y),_)| *y).max().unwrap();

    let mut result = String::new();
    for y in ymin .. ymax+1 {
        for x in xmin .. xmax+2 { // Extra row of black chars to make full characters
            let color = match panel_color(&panels, (x, y)) {
                0 => '.',
                1 => '#',
                _ => panic!()
            };
            result.push(color);
        }
        result.push('\n');
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
