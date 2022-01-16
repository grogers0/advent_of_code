use std::collections::BTreeSet;
use std::io::{self, Read};
use std::ops::{Add, Mul, Sub};

type Coord = i16;

// Row major
#[derive(Clone, Debug)]
struct Matrix([Coord; 9]);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
struct Point {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Matrix {
    const IDENTITY: Matrix = Matrix([1, 0, 0,  0, 1, 0,  0, 0, 1]);
    const ALL_ROTATIONS: [Matrix; 24] = [
        Matrix([ -1,  0,  0,  0, -1,  0,  0,  0,  1 ]),
        Matrix([ -1,  0,  0,  0,  0, -1,  0, -1,  0 ]),
        Matrix([ -1,  0,  0,  0,  0,  1,  0,  1,  0 ]),
        Matrix([ -1,  0,  0,  0,  1,  0,  0,  0, -1 ]),
        Matrix([  0, -1,  0, -1,  0,  0,  0,  0, -1 ]),
        Matrix([  0, -1,  0,  0,  0, -1,  1,  0,  0 ]),
        Matrix([  0, -1,  0,  0,  0,  1, -1,  0,  0 ]),
        Matrix([  0, -1,  0,  1,  0,  0,  0,  0,  1 ]),
        Matrix([  0,  0, -1, -1,  0,  0,  0,  1,  0 ]),
        Matrix([  0,  0, -1,  0, -1,  0, -1,  0,  0 ]),
        Matrix([  0,  0, -1,  0,  1,  0,  1,  0,  0 ]),
        Matrix([  0,  0, -1,  1,  0,  0,  0, -1,  0 ]),
        Matrix([  0,  0,  1, -1,  0,  0,  0, -1,  0 ]),
        Matrix([  0,  0,  1,  0, -1,  0,  1,  0,  0 ]),
        Matrix([  0,  0,  1,  0,  1,  0, -1,  0,  0 ]),
        Matrix([  0,  0,  1,  1,  0,  0,  0,  1,  0 ]),
        Matrix([  0,  1,  0, -1,  0,  0,  0,  0,  1 ]),
        Matrix([  0,  1,  0,  0,  0, -1, -1,  0,  0 ]),
        Matrix([  0,  1,  0,  0,  0,  1,  1,  0,  0 ]),
        Matrix([  0,  1,  0,  1,  0,  0,  0,  0, -1 ]),
        Matrix([  1,  0,  0,  0, -1,  0,  0,  0, -1 ]),
        Matrix([  1,  0,  0,  0,  0, -1,  0,  1,  0 ]),
        Matrix([  1,  0,  0,  0,  0,  1,  0, -1,  0 ]),
        Matrix([  1,  0,  0,  0,  1,  0,  0,  0,  1 ]),
    ];
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0, z: 0 };

    fn manhattan_dist(&self) -> Coord {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl <'a> Mul<Point> for &'a Matrix {
    type Output = Point;
    fn mul(self, p: Point) -> Point {
        let m = &self.0;
        Point {
            x: m[0]*p.x + m[1]*p.y + m[2]*p.z,
            y: m[3]*p.x + m[4]*p.y + m[5]*p.z,
            z: m[6]*p.x + m[7]*p.y + m[8]*p.z,
        }
    }
}

impl <'a> Mul<&Matrix> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        let l = &self.0;
        let r = &rhs.0;
        Matrix([
            l[0]*r[0] + l[1]*r[3] + l[2]*r[6],
            l[0]*r[1] + l[1]*r[4] + l[2]*r[7],
            l[0]*r[2] + l[1]*r[5] + l[2]*r[8],
            l[3]*r[0] + l[4]*r[3] + l[5]*r[6],
            l[3]*r[1] + l[4]*r[4] + l[5]*r[7],
            l[3]*r[2] + l[4]*r[5] + l[5]*r[8],
            l[6]*r[0] + l[7]*r[3] + l[8]*r[6],
            l[6]*r[1] + l[7]*r[4] + l[8]*r[7],
            l[6]*r[2] + l[7]*r[5] + l[8]*r[8],
        ])
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn parse(puzzle_input: &str) -> Vec<BTreeSet<Point>> {
    puzzle_input.split("\n\n")
        .map(|lines| {
            lines.lines().skip(1)
                .map(|line| {
                    let mut iter = line.split(",");
                    let x = iter.next().unwrap().parse().unwrap();
                    let y = iter.next().unwrap().parse().unwrap();
                    let z = iter.next().unwrap().parse().unwrap();
                    assert!(iter.next().is_none());
                    Point { x, y, z }
                })
                .collect()
        })
        .collect()
}

fn detect_overlap(abs_points: &BTreeSet<Point>, other_points: &BTreeSet<Point>, min_overlapping: usize) -> Option<(Matrix, Point)> {
    for rot in Matrix::ALL_ROTATIONS.iter() {
        let other_points: BTreeSet<_> = other_points.iter().map(|&p| rot * p).collect();
        // Not sure if there's a better way to detect overlap than simply trying all possible
        // offsets and checking the points. This algorithm results in a fairly reasonable duration
        // though (~1min)
        for abs_point in abs_points.iter().skip(min_overlapping - 1) {
            for other_point in other_points.iter() {
                let offset = *abs_point - *other_point;
                let other_points: BTreeSet<_> = other_points.iter().map(|&p| p + offset).collect();
                if abs_points.intersection(&other_points).count() >= min_overlapping {
                    return Some((rot.clone(), offset));
                }
            }
        }
    }
    None
}

fn solve(puzzle_input: &str) -> (BTreeSet<Point>, Vec<(Matrix, Point)>) {
    let scanner_points = parse(puzzle_input);
    let num_scanners = scanner_points.len();
    let mut transformations = vec![None; num_scanners];
    let mut beacons = BTreeSet::new();

    // Use scanner 0 as-is as the absolute reference frame
    transformations[0] = Some((Matrix::IDENTITY, Point::ORIGIN));
    beacons.extend(&scanner_points[0]);

    'outer:
    while transformations.iter().any(|r| r.is_none()) {
        for s1 in 0 .. num_scanners {
            if transformations[s1].is_none() { continue }
            for s2 in 0 .. num_scanners {
                if transformations[s2].is_some() { continue }

                if let Some((rot, offset)) = detect_overlap(&scanner_points[s1], &scanner_points[s2], 12) {
                    let rot = &transformations[s1].as_ref().unwrap().0 * &rot;
                    let offset = (&transformations[s1].as_ref().unwrap().0 * offset) + transformations[s1].as_ref().unwrap().1;
                    for p in scanner_points[s2].iter().copied() {
                        beacons.insert((&rot * p) + offset);
                    }
                    transformations[s2] = Some((rot, offset));
                    continue 'outer;
                }
            }
        }

        panic!()
    }

    let transformations = transformations.into_iter().map(Option::unwrap).collect();
    (beacons, transformations)
}

fn part1(beacons: &BTreeSet<Point>) -> usize {
    beacons.len()
}

fn part2(transformations: &[(Matrix, Point)]) -> Coord {
    let mut best_dist = 0;
    for i in 0 .. transformations.len() {
        let pi = transformations[i].1;
        for j in i+1 .. transformations.len() {
            let pj = transformations[j].1;
            let dist = (pi - pj).manhattan_dist();
            if dist > best_dist { best_dist = dist }
        }
    }
    best_dist
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let (beacons, transformations) = solve(&puzzle_input);
    println!("{}", part1(&beacons));
    println!("{}", part2(&transformations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_part1() {
        assert_eq!(79, part1(&solve(EX).0));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3621, part2(&solve(EX).1));
    }
}
