use std::cmp::Ordering;
use std::io::{self, Read};
use std::sync::mpsc::channel;
use std::thread;

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

fn play_game(puzzle_input: &str) -> i64 {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    let mut mem = parse(puzzle_input);
    mem[0] = 2;

    thread::spawn(move || run(&mut mem, &rx_in, tx_out));

    let mut x_paddle = 0;
    let mut x_ball;
    let mut score = 0;
    while let Ok(x) = rx_out.recv() {
        let _y = rx_out.recv().unwrap();
        let id = rx_out.recv().unwrap();
        if x == -1 {
            score = id;
        } else {
            match Tile::from(id) {
                Tile::Ball => {
                    x_ball = x;
                    let joystick = match x_paddle.cmp(&x_ball) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1
                    };
                    tx_in.send(joystick).unwrap();
                },
                Tile::HorizontalPaddle => x_paddle = x,
                _ => ()
            }
        }
    }

    score
}

fn part1(puzzle_input: &str) -> usize {
    let (tx_out, rx_out) = channel();
    run(&mut parse(puzzle_input), &channel().1, tx_out);
    let mut num_blocks = 0;
    while let Ok(_x) = rx_out.recv() {
        let _y = rx_out.recv().unwrap();
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
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}
