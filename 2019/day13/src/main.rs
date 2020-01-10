use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::sync::mpsc::channel;
use std::thread;

use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use intcode::*;

#[derive(Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

impl From<i64> for Tile {
    fn from(id: i64) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!()
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => ".",
            Tile::HorizontalPaddle => "-",
            Tile::Ball => "o"
        })
    }
}

fn play_game(puzzle_input: &str) -> i64 {
    let mut _screen = AlternateScreen::from(io::stdout().into_raw_mode().unwrap());
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    let mut mem = parse(puzzle_input);
    mem[0] = 2;

    let mut progress = OpenOptions::new().read(true).append(true).create(true)
        .open("progress").unwrap();
    let mut replay_str = String::new();
    progress.read_to_string(&mut replay_str).unwrap();
    for ch in replay_str.chars() {
        let joystick = match ch {
            '<' => -1,
            '>' => 1,
            '.' => 0,
            '\n' => continue,
            _ => panic!()
        };
        tx_in.send(joystick).unwrap();
    }

    thread::spawn(move || {
        let mut recent = VecDeque::new();
        let mut keys = io::stdin().keys();
        while let Some(Ok(key)) = keys.next() {
            let (joystick, ch) = match key {
                Key::Esc => break,
                Key::Left | Key::Char('<') => (-1, '<'),
                Key::Right | Key::Char('>') => (1, '>'),
                _ => (0, '.')
            };
            let _ = tx_in.send(joystick);
            recent.push_back(ch);
            if recent.len() > 20 { // Should be enough to recover from a mistake
                let _ = write!(progress, "{}", recent.pop_front().unwrap());
            }
        }
    });
    thread::spawn(move || run(&mut mem, &rx_in, tx_out));

    print!("{}", cursor::Goto(1, 1));
    let mut score: i64 = 0;
    while let Ok(x) = rx_out.recv() {
        let y = rx_out.recv().unwrap();
        let id = rx_out.recv().unwrap();
        if x == -1 {
            score = id;
            continue;
        }
        print!("{}{}{}",
            cursor::Goto(x as u16 + 1, y as u16 + 1),
            Tile::from(id),
            cursor::Goto(1, 1));
        io::stdout().flush();
    }

    score
}

fn part1(puzzle_input: &str) -> usize {
    let (tx_out, rx_out) = channel();
    run(&mut parse(puzzle_input), &channel().1, tx_out);
    let mut num_blocks = 0;
    while let Ok(x) = rx_out.recv() {
        let y = rx_out.recv().unwrap();
        if let Tile::Block = rx_out.recv().unwrap().into() {
            num_blocks += 1;
        }
    }
    num_blocks
}

fn part2(puzzle_input: &str) -> i64 {
    play_game(puzzle_input)
}

fn main() {
    let mut puzzle_input = String::new();
    File::open("input").unwrap().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}
