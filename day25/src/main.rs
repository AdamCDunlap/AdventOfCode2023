use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Node([u8; 3]);

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &std::str::from_utf8(&self.0).unwrap())
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.as_bytes().try_into().map_err(|_| ())?))
    }
}

#[derive(Debug, Clone)]
struct Graph {
    adjacency_list: HashMap<Node, HashSet<Node>>,
    // adjacenecy_matrix: Vec<Vec<bool>>,
}

impl Graph {
    fn from_str(input: &str) -> Graph {
        let mut adjacency_list: HashMap<Node, HashSet<Node>> = HashMap::new();
        for line in input.lines() {
            let parts: Vec<_> = line.split(':').collect();
            let start: Node = parts[0].trim().parse().unwrap();
            let ends = parts[1].trim().split_whitespace();
            for end in ends {
                let end: Node = end.parse().unwrap();
                adjacency_list
                    .entry(start.clone())
                    .and_modify(|set| {
                        set.insert(end.clone());
                    })
                    .or_insert_with(|| HashSet::from([end.clone()]));

                adjacency_list
                    .entry(end.clone())
                    .and_modify(|set| {
                        set.insert(start.clone());
                    })
                    .or_insert_with(|| HashSet::from([start.clone()]));
            }
        }

        Graph { adjacency_list }
    }
}

fn has_at_least_n_unique_paths(
    graph: &Graph,
    start: Node,
    end: Node,
    unique_path_threshold: usize,
) -> bool {
    let mut paths: HashMap<(Node, u8), Vec<Node>> = HashMap::new();

    let mut to_examine: VecDeque<(Node, u8)> = VecDeque::from([(start, 0)]);
    paths.insert((start, 0), vec![]);

    while let Some((here, path_idx)) = to_examine.pop_front() {
        let path_key = (here, path_idx);

        let mut check_next = |next: Node| {
            let mut path_to_next = paths[&path_key].clone();
            path_to_next.push(next);
            let mut changed = false;
            match paths.entry((next, path_idx + 1)) {
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    if e.get().len() > path_to_next.len() {
                        e.insert(path_to_next);
                        changed = true;
                    }
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(path_to_next);
                    changed = true;
                }
            }

            if changed {
                to_examine.push_back((next, path_idx + 1));
            }
        };

        if here == end {
            // We got a path to the end, now start over at the start to see how many loops can be made.
            if path_idx as usize >= unique_path_threshold {
                return true;
            }
            check_next(start);
        }

        for next in graph.adjacency_list.get(&here).unwrap().iter() {
            if *next != end && paths[&path_key].iter().find(|n| **n == *next).is_some() {
                // The path already has this node (it's OK if it's the end, though)
                continue;
            }

            let mut check_next = |next: Node| {
                let mut path_to_next = paths[&path_key].clone();
                path_to_next.push(next);
                let mut changed = false;
                match paths.entry((next, path_idx + 1)) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        if e.get().len() > path_to_next.len() {
                            e.insert(path_to_next);
                            changed = true;
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(path_to_next);
                        changed = true;
                    }
                }

                if changed {
                    to_examine.push_back((next, path_idx + 1));
                }
            };
            check_next(*next)
        }
    }

    false
}

#[test]
fn test_from_str() {
    let graph = Graph::from_str(TEST_INPUT);
    dbg!(&graph);
}

fn find_nodes_in_loosely_connected_parts(graph: &Graph) -> (Node, Node) {
    for &n1 in graph.adjacency_list.keys() {
        for &n2 in graph.adjacency_list.keys() {
            if n1 == n2 {
                continue;
            }
            if !has_at_least_n_unique_paths(graph, n1, n2, 3) {
                return (n1, n2);
            }
        }
    }
    unreachable!("No loosely connected parts")
}

#[test]
fn test_find_nodes_in_loosely_connected_parts() {
    let graph = Graph::from_str(TEST_INPUT);

    dbg!(find_nodes_in_loosely_connected_parts(&graph));
}

fn find_edges_to_disconnect(graph: &Graph) -> [(Node, Node); 3] {
    todo!()
}

// fn part1(input: &str) -> usize {

// }

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    let graph = Graph::from_str(input);
    println!(
        "Number of nodes: {} Number of edges: {}",
        graph.adjacency_list.len(),
        graph
            .adjacency_list
            .values()
            .map(|v| v.len())
            .sum::<usize>()
    );
}

const TEST_INPUT: &str = r"jqt: rhn xhk nvd
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
