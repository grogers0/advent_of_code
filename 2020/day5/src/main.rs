use std::io::{self, Read};

struct Seat {
    row: usize,
    col: usize
}

const TOTAL_SEATS: usize = 128*8;

impl Seat {
    fn parse(s: &str) -> Seat {
        let mut row = 0;
        let mut col = 0;
        for ch in s.chars() {
            match ch {
                'F' => row *= 2,
                'B' => row = row*2 + 1,
                'L' => col *= 2,
                'R' => col = col*2 + 1,
                _ => panic!()
            }
        }
        assert!(row < 128 && col < 8);
        Seat { row: row, col: col }
    }

    fn id(&self) -> usize {
        self.row*8 + self.col
    }
}

fn part1(seat_ids: &Vec<usize>) -> usize {
    *seat_ids.iter().max().unwrap()
}

fn part2(seat_ids: &Vec<usize>) -> usize {
    let mut filled_seats: [bool; TOTAL_SEATS] = [false; TOTAL_SEATS];
    for id in seat_ids {
        filled_seats[*id] = true;
    }
    for id in 1..TOTAL_SEATS {
        if !filled_seats[id] && filled_seats[id+1] && filled_seats[id-1] {
            return id
        }
    }
    panic!()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let seat_ids = puzzle_input.lines()
        .map(|line| Seat::parse(line).id())
        .collect();

    println!("{}", part1(&seat_ids));
    println!("{}", part2(&seat_ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_parsing() {
        let seat = Seat::parse("FBFBBFFRLR");
        assert_eq!(44, seat.row);
        assert_eq!(5, seat.col);
        assert_eq!(357, seat.id());

        let seat = Seat::parse("BFFFBBFRRR");
        assert_eq!(70, seat.row);
        assert_eq!(7, seat.col);
        assert_eq!(567, seat.id());

        let seat = Seat::parse("FFFBBBFRRR");
        assert_eq!(14, seat.row);
        assert_eq!(7, seat.col);
        assert_eq!(119, seat.id());

        let seat = Seat::parse("BBFFBBFRLL");
        assert_eq!(102, seat.row);
        assert_eq!(4, seat.col);
        assert_eq!(820, seat.id());
    }
}
