use std::cell::Cell;
use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, RecvTimeoutError, Sender, TryRecvError};
use std::time::{Duration, Instant};
use std::thread;

use intcode::*;

struct PacketReceiver {
    receiver: Receiver<(i64, i64)>,
    next: Cell<Option<i64>>
}

impl Input for PacketReceiver {
    fn recv(&self) -> Result<i64, RecvError> {
        if let Some(y) = self.next.get() {
            self.next.set(None);
            return Ok(y)
        }
        // It'd be nice not to busy loop here with something like recv_timeout, but it causes
        // panics here, see https://github.com/rust-lang/rust/issues/39364
        match self.receiver.try_recv() {
            Ok((x, y)) => {
                self.next.set(Some(y));
                Ok(x)
            },
            Err(TryRecvError::Empty) => Ok(-1),
            Err(TryRecvError::Disconnected) => Err(RecvError)
        }
    }
}

fn build_network(puzzle_input: &str) -> (
    Receiver<(i64, i64)>,
    HashMap<i64, Sender<(i64, i64)>>,
    Arc<Mutex<Cell<Instant>>>) {

    let last_packet_time = Arc::new(Mutex::new(Cell::new(Instant::now())));
    let mut packet_queues = HashMap::new();
    let mut packet_routers = Vec::new();
    for i in 0 .. 50 {
        let (tx_in, rx_in) = channel();
        let (tx_out, rx_out) = channel();

        let mut mem = parse(puzzle_input);
        let rx_in = PacketReceiver { receiver: rx_in, next: Cell::new(Some(i)) };
        thread::spawn(move || run(&mut mem, &rx_in, tx_out));

        packet_queues.insert(i, tx_in);
        packet_routers.push(rx_out);
    }
    let (tx_nat, rx_nat) = channel();
    packet_queues.insert(255, tx_nat);

    for receiver in packet_routers {
        let packet_queues = packet_queues.clone();
        let last_packet_time = last_packet_time.clone();
        thread::spawn(move || {
            while let Ok(dest) = receiver.recv() {
                let x = receiver.recv().unwrap();
                let y = receiver.recv().unwrap();
                packet_queues.get(&dest).unwrap().send((x, y)).unwrap();
                last_packet_time.lock().unwrap().set(Instant::now());
            }
        });
    }

    (rx_nat, packet_queues, last_packet_time)
}

fn part1(puzzle_input: &str) -> i64 {
    let (rx_nat, _, _) = build_network(puzzle_input);
    let (_x, y) = rx_nat.recv().unwrap();
    y
}

const IDLE_TIME_MILLIS: u64 = 500;

fn part2(puzzle_input: &str) -> i64 {
    let (rx_nat, packet_queues, last_packet_time) = build_network(puzzle_input);
    let mut last_packet_in = None;
    let mut last_packet_out = None;

    loop {
        let start_time = last_packet_time.lock().unwrap().get().elapsed();
        let timeout = if start_time >= Duration::from_millis(IDLE_TIME_MILLIS) {
            Duration::from_millis(0)
        } else {
            Duration::from_millis(IDLE_TIME_MILLIS) - start_time
        };
        match rx_nat.recv_timeout(timeout) {
            Ok(packet) => {
                last_packet_in = Some(packet);
                continue
            },
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => panic!()
        }

        if last_packet_time.lock().unwrap().get().elapsed().as_millis() > IDLE_TIME_MILLIS as u128 {
            let (x, y) = last_packet_in.unwrap();
            match last_packet_out {
                Some((_, y2)) if y == y2 => return y,
                _ => ()
            }
            last_packet_out = last_packet_in;
            packet_queues.get(&0).unwrap().send((x, y)).unwrap();
            last_packet_time.lock().unwrap().set(Instant::now());
        }
    }
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}
