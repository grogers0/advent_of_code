use std::io::{self, Read};

struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<usize>
}

fn parse(input: &str) -> Node {
    let mut iter = input.trim_end().split(' ');
    let ret = parse_iter(&mut iter);
    assert_eq!(None, iter.next());
    ret
}

fn parse_iter(input: &mut Iterator<Item=&str>) -> Node {
    let num_children = input.next().unwrap().parse().unwrap();
    let num_metadata = input.next().unwrap().parse().unwrap();
    let mut children = Vec::new();
    for _ in 0..num_children {
        children.push(Box::new(parse_iter(input)));
    }
    let mut metadata = Vec::new();
    for _ in 0..num_metadata {
        metadata.push(input.next().unwrap().parse().unwrap());
    }
    Node {
        children: children,
        metadata: metadata
    }
}

fn sum_part1(node: &Node) -> usize {
    let mut sum = node.metadata.iter().sum();
    for child in node.children.iter() {
        sum += sum_part1(child);
    }
    sum
}

fn part1(input: &str) -> usize {
    sum_part1(&parse(input))
}

fn sum_part2(node: &Node) -> usize {
    if node.children.is_empty() {
        node.metadata.iter().sum()
    } else {
        node.metadata.iter()
            .filter(|m| **m > 0 && **m <= node.children.len())
            .map(|m| sum_part2(&node.children[*m - 1]))
            .sum()
    }
}

fn part2(input: &str) -> usize {
    sum_part2(&parse(input))
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

    const EX: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 66);
    }

}
