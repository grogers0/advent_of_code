use std::collections::{HashSet, HashMap};
use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use intcode::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Dir { North, South, East, West }

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Dir::North => "north",
            Dir::South => "south",
            Dir::East => "east",
            Dir::West => "west"
        }
    }
}

impl From<&str> for Dir {
    fn from(s: &str) -> Dir {
        match s {
            "north" => Dir::North,
            "south" => Dir::South,
            "east" => Dir::East,
            "west" => Dir::West,
            _ => panic!()
        }
    }
}

lazy_static! {
    static ref DEADLY_ITEMS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("escape pod");
        s.insert("giant electromagnet");
        s.insert("infinite loop");
        s.insert("molten lava");
        s.insert("photons");
        s
    };
}

#[derive(Debug)]
struct Room {
    name: String,
    password: Option<String>,
    doors: Vec<Dir>,
    items: Vec<String>,
    ejected: bool
}

impl From<&str> for Room {
    fn from(s: &str) -> Room {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new("^== (.*) ==$").unwrap();
            static ref LIST_RE: Regex = Regex::new("^- (.*)$").unwrap();
            static ref PASSWORD_RE: Regex = Regex::new("You should be able to get in by typing (.+) on the keypad at the main airlock").unwrap();
        }
        let mut name = None;
        let mut password = None;
        let mut doors = Vec::new();
        let mut items = Vec::new();
        let mut scanning_description = false;
        let mut scanning_doors = false;
        let mut scanning_items = false;
        let mut scanning_password = false;
        let mut ejected = false;
        for line in s.lines() {
            if scanning_description {
                scanning_description = false;
            } else if line.trim().is_empty() {
                scanning_doors = false; scanning_items = false;
            } else if scanning_doors || scanning_items {
                let cap = LIST_RE.captures(line).unwrap();
                if scanning_doors {
                    doors.push(Dir::from(&cap[1]));
                } else {
                    items.push(cap[1].to_string());
                }
            } else if let Some(cap) = NAME_RE.captures(line) {
                name = Some(cap[1].to_string());
                scanning_description = true;
            } else if line == "Doors here lead:" {
                scanning_doors = true;
            } else if line == "Items here:" {
                scanning_items = true;
            } else if line.ends_with("you are ejected back to the checkpoint.") {
                name = None;
                doors = Vec::new();
                ejected = true;
            } else if line.ends_with("and you enter the cockpit.") {
                scanning_password = true;
            } else if scanning_password {
                if let Some(cap) = PASSWORD_RE.captures(line) {
                    password = Some(cap[1].to_string());
                }
            } else {
                panic!(format!("misunderstood string: '{}' part of '{}'", line, s));
            }
        }
        Room {
            name: name.unwrap(),
            password: password,
            doors: doors,
            items: items,
            ejected: ejected
        }
    }
}

fn read_room(rx: &Receiver<i64>) -> Room {
    let mut output = String::new();
    while let Ok(line) = recv_line(rx) {
        if line == "Command?" { break }
        output.push_str(&line);
        output.push('\n');
    }
    Room::from(output.as_str())
}

fn explore_and_take_items(tx: &Sender<i64>, rx: &Receiver<i64>) -> (Vec<String>, Dir) {
    let mut items = Vec::new();
    let mut path = Vec::<Dir>::new();
    let mut path_to_checkpoint = None;
    let mut dir_from_checkpoint = None;
    let mut doors_tried: HashMap<String, HashSet<Dir>> = HashMap::new();
    let mut seen = HashSet::new();
    loop {
        let room = read_room(rx);
        if room.name == "Security Checkpoint" {
            path_to_checkpoint = Some(path.clone());
            dir_from_checkpoint = room.doors.iter()
                .filter(|&&dir| dir != path[path.len() - 1].opposite())
                .cloned().next();
        } else {
            for item in room.items.iter().filter(|item| !DEADLY_ITEMS.contains(item.as_str())) {
                items.push(item.to_string());
                send_line(tx, &format!("take {}", item));
                while let Ok(line) = recv_line(rx) {
                    if line == "Command?" { break }
                }
            }

            if seen.insert(room.name.clone()) {
                doors_tried.insert(room.name.clone(), HashSet::new());
            }

            if let Some(dir) = room.doors.iter().filter(|dir| !doors_tried[&room.name].contains(dir)).cloned().next() {
                doors_tried.get_mut(&room.name).unwrap().insert(dir);
                path.push(dir);
                send_line(tx, dir.as_str());
                continue;
            }
        }

        if let Some(dir) = path.pop() {
            send_line(tx, dir.opposite().as_str());
        } else {
            break;
        }
    }

    for dir in path_to_checkpoint.unwrap() {
        send_line(tx, dir.as_str());
        while let Ok(line) = recv_line(rx) {
            if line == "Command?" { break }
        }
    }
    (items, dir_from_checkpoint.unwrap())
}

fn crack_checkpoint(tx: &Sender<i64>, rx: &Receiver<i64>, items: Vec<String>, dir: Dir) -> String {
    let mut carrying = (1u64 << items.len()) - 1;
    for desired in 0 .. (1u64 << items.len()) {
        let dropping = (carrying ^ desired) & carrying;
        let taking = (carrying ^ desired) & !carrying;
        carrying = desired;
        for (i, item) in items.iter().enumerate() {
            if dropping & (1u64 << i) != 0 {
                send_line(tx, &format!("drop {}", item));
                while let Ok(line) = recv_line(rx) {
                    if line == "Command?" { break }
                }
            } else if taking & (1u64 << i) != 0 {
                send_line(tx, &format!("take {}", item));
                while let Ok(line) = recv_line(rx) {
                    if line == "Command?" { break }
                }
            }
        }

        send_line(tx, dir.as_str());
        let room = read_room(rx);
        if !room.ejected {
            return room.password.unwrap();
        }
    }
    panic!();
}

fn part1(puzzle_input: &str) -> String {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();

    let mut mem = parse(puzzle_input);
    thread::spawn(move || run(&mut mem, &rx_in, tx_out));

    let (items, dir) = explore_and_take_items(&tx_in, &rx_out);
    crack_checkpoint(&tx_in, &rx_out, items, dir)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
}
