use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use intcode::*;

// TODO - this would be a good time to learn about async/await
fn connect_chain(mem_str: &str, phases: Vec<i64>) -> (Sender<i64>, Receiver<i64>) {
    let (mut tx, mut rx) = channel();
    let tx_init = tx.clone();

    for phase in phases {
        let rx_in = rx;
        tx.send(phase).unwrap();

        let (tx_tmp, rx_tmp) = channel();
        tx = tx_tmp; rx = rx_tmp;
        let tx_out = tx.clone();

        let mut mem = parse(mem_str);
        thread::spawn(move || run(&mut mem, rx_in, tx_out));
    }

    // NOTE - The initial input must be sent after all phases have been sent, otherwise the output
    // of one program might be sent before the phase for that program has been sent.
    tx_init.send(0).unwrap();
    (tx_init, rx)
}

fn part1(mem_str: &str) -> i64 {
    permutohedron::Heap::new(&mut (0..5).collect::<Vec<_>>()).map(|phases| {
        connect_chain(mem_str, phases).1.recv().unwrap()
    }).max().unwrap()
}

fn part2(mem_str: &str) -> i64 {
    permutohedron::Heap::new(&mut (5..10).collect::<Vec<_>>()).map(|phases| {
        let (tx_in, rx_out) = connect_chain(mem_str, phases);
        let mut output = None;
        while let Ok(val) = rx_out.recv() {
            let _ = tx_in.send(val); // Ok if the first program has stopped
            output = Some(val);
        }
        output.unwrap()
    }).max().unwrap()
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
    fn test_part1() {
        assert_eq!(part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);
        assert_eq!(part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);
        assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
        assert_eq!(part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
