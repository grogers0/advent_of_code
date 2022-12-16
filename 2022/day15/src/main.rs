use std::collections::HashSet;
use std::io::{self, Read};

use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

struct Measurement {
    sensor: Pos,
    beacon: Pos,
    sensor_range: i32,
}


fn parse(puzzle_input: &str) -> Vec<Measurement> {
    let re = Regex::new("^Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)$").unwrap();
    puzzle_input.trim_end().lines().map(|line| {
        let cap = re.captures(line).unwrap();
        let sensor_x = cap[1].parse().unwrap();
        let sensor_y = cap[2].parse().unwrap();
        let sensor = Pos::new(sensor_x, sensor_y);
        let beacon_x = cap[3].parse().unwrap();
        let beacon_y = cap[4].parse().unwrap();
        let beacon = Pos::new(beacon_x, beacon_y);
        Measurement { sensor, beacon, sensor_range: manhattan_dist(sensor, beacon) }
    }).collect()
}

fn manhattan_dist(pos1: Pos, pos2: Pos) -> i32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

fn count_impossible_beacons(measurements: &[Measurement], row: i32) -> usize {
    // TODO - Could definitely use an interval set to speed this up, but it's good enough
    let mut xs = HashSet::new();
    for m in measurements {
        let diff = m.sensor_range - (row - m.sensor.y).abs();
        if diff >= 0 {
            for x in (m.sensor.x - diff)..=(m.sensor.x + diff) {
                xs.insert(x);
            }
        }
    }
    for m in measurements {
        if m.beacon.y == row {
            xs.remove(&m.beacon.x);
        }
    }
    xs.len()
}

fn in_range(meas: &Measurement, pos: Pos) -> bool {
    manhattan_dist(meas.sensor, pos) <= meas.sensor_range
}

// Since there is a unique location, the distress beacon must be at the edge of some sensor's
// range, so just iterate all those points and check if it satisfies the constraints
fn find_distress_beacon(measurements: &[Measurement], max: i32) -> i64 {
    let check_pos = |pos: Pos| -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x <= max && pos.y <= max &&
            !measurements.iter().any(|m| in_range(m, pos))
    };
    let tuning_freq = |pos: Pos| -> i64 {
        pos.x as i64 * 4000000 + pos.y as i64
    };

    for m in measurements {
        let mut pos = Pos::new(m.sensor.x, m.sensor.y - m.sensor_range - 1);
        while pos.y < m.sensor.y {
            if check_pos(pos) { return tuning_freq(pos); }
            pos.x -= 1;
            pos.y += 1;
        }
        while pos.x < m.sensor.x {
            if check_pos(pos) { return tuning_freq(pos); }
            pos.x += 1;
            pos.y += 1;
        }
        while pos.y > m.sensor.y {
            if check_pos(pos) { return tuning_freq(pos); }
            pos.x += 1;
            pos.y -= 1;
        }
        while pos.x > m.sensor.x {
            if check_pos(pos) { return tuning_freq(pos); }
            pos.x -= 1;
            pos.y -= 1;
        }
    }
    panic!()
}

fn part1(measurements: &[Measurement]) -> usize {
    count_impossible_beacons(measurements, 2000000)
}

fn part2(measurements: &[Measurement]) -> i64 {
    find_distress_beacon(measurements, 4000000)
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let measurements = parse(&puzzle_input);
    println!("{}", part1(&measurements));
    println!("{}", part2(&measurements));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part1() {
        assert_eq!(count_impossible_beacons(&parse(EX), 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_distress_beacon(&parse(EX), 20), 56000011);
    }
}
