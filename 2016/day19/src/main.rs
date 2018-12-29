use std::io::{self, Read};

use linked_list::{LinkedList, Cursor};

fn next_circular(cursor: &mut Cursor<usize>) -> usize {
    if let Some(elem) = cursor.next() {
        *elem
    } else {
        *cursor.next().unwrap()
    }
}

fn remove_circular(cursor: &mut Cursor<usize>) -> usize {
    if let Some(elem) = cursor.remove() {
        elem
    } else {
        cursor.next();
        cursor.remove().unwrap()
    }
}

fn build_circle(num_elems: usize) -> LinkedList<usize> {
    let mut circle = LinkedList::new();
    for i in 1..=num_elems {
        circle.push_back(i);
    }
    circle    
}

fn part1(starting_cnt: usize) -> usize {
    let mut circle = build_circle(starting_cnt);
    let mut cur = circle.cursor();
    next_circular(&mut cur);
    for _ in 1..starting_cnt {
        remove_circular(&mut cur);
        next_circular(&mut cur);
    }
    circle.pop_front().unwrap()
}

fn part2(starting_cnt: usize) -> usize {
    let mut circle = build_circle(starting_cnt);
    let mut cur = circle.cursor();
    cur.seek_forward(starting_cnt/2);
    for i in 1..starting_cnt {
        remove_circular(&mut cur);
        if i % 2 == starting_cnt % 2 { next_circular(&mut cur); }
    }
    circle.pop_front().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let starting_cnt = input.trim_end().parse().unwrap();

    println!("{}", part1(starting_cnt));
    println!("{}", part2(starting_cnt));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(5), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(5), 2);
    }
}
