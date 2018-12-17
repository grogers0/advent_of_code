use std::collections::BTreeSet;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    Left,
    Straight,
    Right
}

#[derive(Clone, Debug)]
struct Cart {
    position: (usize, usize),
    direction: (i32, i32),
    next_turn: Turn
}

// Returns (carts, track)
fn parse(input: &str) -> (Vec<Cart>, Vec<Vec<char>>) {
    let mut carts = Vec::new();
    let mut track = Vec::new();

    for (y,line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x,ch) in line.chars().enumerate() {
            match ch {
                '^' => {
                    carts.push(Cart { position: (x, y), direction: (0, -1), next_turn: Turn::Left });
                    row.push('|');
                },
                'v' => {
                    carts.push(Cart { position: (x, y), direction: (0, 1), next_turn: Turn::Left });
                    row.push('|');
                },
                '<' => {
                    carts.push(Cart { position: (x, y), direction: (-1, 0), next_turn: Turn::Left });
                    row.push('-');
                },
                '>' => {
                    carts.push(Cart { position: (x, y), direction: (1, 0), next_turn: Turn::Left });
                    row.push('-');
                },
                ' ' | '+' | '-' | '|' | '\\' | '/' => row.push(ch),
                _ => unreachable!()
            }
        }

        track.push(row);
    }
    let width = track[1].len();
    for row in track.iter() {
        assert_eq!(width, row.len());
    }
    (carts, track)
}

fn update_position(pos: &mut (usize, usize), dir: &(i32, i32)) {
    if dir.0 < 0 {
        pos.0 -= dir.0.abs() as usize;
    } else {
        pos.0 += dir.0 as usize;
    }
    if dir.1 < 0 {
        pos.1 -= dir.1.abs() as usize;
    } else {
        pos.1 += dir.1 as usize;
    }
}

fn turn_left(dir: &mut (i32, i32)) {
    if dir.0 == 0 { // vertical
        *dir = (dir.1, 0);
    } else { // horizontal
        *dir = (0, -dir.0);
    }
}

fn turn_right(dir: &mut (i32, i32)) {
    if dir.0 == 0 { // vertical
        *dir = (-dir.1, 0);
    } else { // horizontal
        *dir = (0, dir.0);
    }
}

fn move_cart(cart: &mut Cart, track: &Vec<Vec<char>>) {
    match track[cart.position.1][cart.position.0] {
        '-' | '|' => (),
        '\\' if cart.direction.0 == 0 => turn_left(&mut cart.direction),
        '\\' if cart.direction.1 == 0 => turn_right(&mut cart.direction),
        '/' if cart.direction.0 == 0 => turn_right(&mut cart.direction),
        '/' if cart.direction.1 == 0 => turn_left(&mut cart.direction),
        '+' if cart.next_turn == Turn::Left => {
            turn_left(&mut cart.direction);
            cart.next_turn = Turn::Straight;
        },
        '+' if cart.next_turn == Turn::Straight => cart.next_turn = Turn::Right,
        '+' if cart.next_turn == Turn::Right => {
            turn_right(&mut cart.direction);
            cart.next_turn = Turn::Left;
        },
        _ => unreachable!()
    }
    update_position(&mut cart.position, &cart.direction);
}

fn is_collision(collision_pos: (usize, usize), carts: &Vec<Cart>) -> bool {
    carts.iter()
        .map(|cart| cart.position)
        .filter(|pos| *pos == collision_pos)
        .count() > 1
}

fn part1(input: &str) -> String {
    let (mut carts, track) = parse(input);

    loop {
        carts.sort_by_key(|c| (c.position.1, c.position.0));

        for i in 0..carts.len() {
            move_cart(&mut carts[i], &track);
            let pos = carts[i].position;
            if is_collision(pos, &carts) {
                return format!("{},{}", pos.0, pos.1);
            }
        }
    }
}

fn part2(input: &str) -> String {
    let (mut carts, track) = parse(input);

    loop {
        carts.sort_by_key(|c| (c.position.1, c.position.0));

        let mut collided = BTreeSet::new();
        for i in 0..carts.len() {
            if collided.contains(&i) {
                continue;
            }
            move_cart(&mut carts[i], &track);
            let pos = carts[i].position;
            if is_collision(pos, &carts) {
                for (j, cart) in carts.iter().enumerate() {
                    if cart.position == pos {
                        collided.insert(j);
                    }
                }
            }
        }
        let mut non_collided_carts = Vec::new();
        for (i, cart) in carts.into_iter().enumerate() {
            if !collided.contains(&i) {
                non_collided_carts.push(cart);
            }
        }
        carts = non_collided_carts;

        if carts.len() == 1 {
            return format!("{},{}", carts[0].position.0, carts[0].position.1);
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
        let ex =
r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
        assert_eq!(part1(ex), "7,3");
    }

    #[test]
    fn test_part2() {
        let ex =
r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
        assert_eq!(part2(ex), "6,4");
    }

}
