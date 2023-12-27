use std::collections::{HashSet, HashMap, VecDeque};
use std::cmp::{min, max, Reverse};
use std::io::{self, Read};

use rand::random;

type Id = u16;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Edge(Id, Id);
type Graph = HashMap<Id, HashSet<Id>>;

impl Edge {
    fn new(v1: Id, v2: Id) -> Edge {
        Edge(min(v1, v2), max(v1, v2))
    }
}

fn parse(puzzle_input: &str) -> Graph {
    let mut graph = HashMap::new();
    let mut vertex_ids = HashMap::new();
    let mut id_counter: Id = 0;
    let mut to_id = |v_str: &str| -> Id {
        *vertex_ids.entry(v_str.to_string()).or_insert_with(|| {
            let id = id_counter;
            id_counter = id_counter.checked_add(1).unwrap();
            id
        })
    };
    for line in puzzle_input.lines() {
        let mut parts = line.split(": ");
        let v1 = to_id(parts.next().unwrap());
        for v2 in parts.next().unwrap().split(" ").map(|s| to_id(s)) {
            add_edge(&mut graph, Edge::new(v1, v2));
        }
    }
    graph
}

fn add_edge(graph: &mut Graph, edge: Edge) {
    graph.entry(edge.0).or_insert(HashSet::new()).insert(edge.1);
    graph.entry(edge.1).or_insert(HashSet::new()).insert(edge.0);
}

fn remove_edge(graph: &mut Graph, edge: Edge) {
    fn remove_edge_unidirectional(graph: &mut Graph, v1: Id, v2: Id) {
        let edges = graph.get_mut(&v1).unwrap();
        edges.remove(&v2);
        if edges.is_empty() {
            graph.remove(&v1);
        }
    }
    remove_edge_unidirectional(graph, edge.0, edge.1);
    remove_edge_unidirectional(graph, edge.1, edge.0);
}

fn pick_two_random_vertices(num_vertices: Id) -> (Id, Id) {
    loop {
        let v1 = random::<Id>() % num_vertices;
        let v2 = random::<Id>() % num_vertices;
        if v1 == v2 { continue }
        return (min(v1, v2), max(v1, v2));
    }
}

fn find_path(graph: &Graph, v_start: Id, v_end: Id) -> Vec<Edge> {
    let mut queue = VecDeque::new();
    queue.push_back((v_start, Vec::new(), HashSet::new()));
    while let Some((v1, path, seen)) = queue.pop_front() {
        if seen.contains(&v1) { continue }
        if v1 == v_end { return path }
        let mut seen = seen;
        seen.insert(v1);
        for v2 in &graph[&v1] {
            let mut path = path.clone();
            path.push(Edge::new(v1, *v2));
            queue.push_back((*v2, path, seen.clone()));
        }
    }
    panic!()
}

fn all_vertices(graph: &Graph) -> HashSet<Id> {
    let mut vertices = HashSet::new();
    for (v1, v2s) in graph {
        vertices.insert(*v1);
        for v2 in v2s {
            vertices.insert(*v2);
        }
    }
    vertices
}

fn get_connected_groups(graph: &Graph, vertices: &HashSet<Id>) -> Vec<HashSet<Id>> {
    let mut groups = Vec::new();
    let mut seen = HashSet::new();
    for &v in vertices {
        if seen.contains(&v) { continue }
        let mut stack = Vec::new();
        let mut group = HashSet::new();
        stack.push(v);
        while let Some(v) = stack.pop() {
            if !seen.insert(v) { continue }
            group.insert(v);
            for v2 in &graph[&v] {
                stack.push(*v2);
            }
        }
        groups.push(group);
    }
    groups
}

fn part1(mut graph: Graph) -> usize {
    let vertices = all_vertices(&graph);

    // Given a collections of random pairs of points, the min cut edges are likely to be overly
    // represented in the shortest paths between them. A higher number of pairs will produce a more
    // likely chance to find them, but take longer to run.
    let mut edge_weights = HashMap::new();
    for _ in 0..100 {
        let (v1, v2) = pick_two_random_vertices(vertices.len() as Id);
        for edge in find_path(&graph, v1, v2) {
            *edge_weights.entry(edge).or_insert(0) += 1;
        }
    }
    let mut edge_weights: Vec<_> = edge_weights.into_iter().collect();
    edge_weights.sort_unstable_by_key(|&(_e, cnt)| Reverse(cnt));
    // The top three edges may not be the exact min cuts, but an exhaustive search through the top
    // N will certainly find them if N is big enough. Too big and it will take longer to run.
    let edges: Vec<_> = edge_weights[0..min(10, edge_weights.len())]
        .into_iter().map(|(e, _)| e).collect();

    for i in 0..(edges.len()-2) {
        remove_edge(&mut graph, *edges[i]);
        for j in (i+1)..(edges.len()-1) {
            remove_edge(&mut graph, *edges[j]);
            for k in (j+1)..(edges.len()-2) {
                remove_edge(&mut graph, *edges[k]);
                let groups = get_connected_groups(&graph, &vertices);
                if groups.len() == 2 {
                    return groups[0].len() * groups[1].len();
                }
                add_edge(&mut graph, *edges[k]);
            }
            add_edge(&mut graph, *edges[j]);
        }
        add_edge(&mut graph, *edges[i]);
    }
    panic!("Randomized search failed to find a solution, increase repetitions")
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    let graph = parse(&puzzle_input);
    println!("{}", part1(graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(EX)), 54);
    }
}
