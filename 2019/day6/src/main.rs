use std::collections::HashMap;
use std::io::{self, Read};

// planet -> what it orbits
fn parse(input: &str) -> HashMap<&str, &str> {
    input.trim().lines().map(|line| {
        let mut sp = line.split(")");
        let cen = sp.next().unwrap();
        let obj = sp.next().unwrap();
        assert_eq!(None, sp.next());
        (obj, cen)
    }).collect()
}

fn part1(input: &str) -> usize {
    let orbits = parse(input);
    let mut cnt = 0;
    for (_, mut cen) in orbits.iter() {
        cnt += 1;
        while let Some(new_cen) = orbits.get(cen) {
            cnt += 1;
            cen = new_cen;
        }
    }
    cnt
}

fn part2(input: &str) -> usize {
    let orbits = parse(input);
    let mut src = orbits.get("YOU").unwrap();
    let mut dst = orbits.get("SAN").unwrap();
    let mut src_trans = HashMap::<&str,usize>::new();
    src_trans.insert(src, 0);
    let mut n = 0;
    while let Some(cen) = orbits.get(src) {
        n += 1;
        src_trans.insert(cen, n);
        src = cen;
    }
    let mut n = 0;
    loop {
        if let Some(src_n) = src_trans.get(dst) {
            return n + src_n;
        } else if let Some(cen) = orbits.get(dst) {
            n += 1;
            dst = cen;
        } else {
            unreachable!(); // It's a DAG so we'll always meet at least at the center
        }
    }
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
        let ex = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        assert_eq!(part1(ex), 42);
    }

    #[test]
    fn test_part2() {
        let ex = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(part2(ex), 4);
    }
}
