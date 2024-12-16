use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::BufRead;

use compact_str::CompactString;
use smallvec::SmallVec;

use crate::day::CombinedSolver;

pub struct Day12;

impl CombinedSolver for Day12 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut graph = Graph::new();

        for (a, b) in input.lines().map(Result::unwrap).map(|s| {
            let (a, b) = s.split_once("-").unwrap();
            (Node::from(a), Node::from(b))
        }) {
            graph.entry(a.clone()).or_default().push(b.clone());
            graph.entry(b).or_default().push(a);
        }

        let part_1 = score_1(State::start(), &graph);
        let mut memo = HashMap::new();
        let part_2 = score_2(State2::start(), &graph, &mut memo);

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

pub type Node = CompactString;

pub type Graph = HashMap<Node, SmallVec<[Node; 6]>>;

pub struct State {
    current: Node,
    little_visited: HashSet<Node>,
}

impl State {
    pub fn start() -> Self {
        Self {
            current: Node::new("start"),
            little_visited: HashSet::from([Node::new("start")]),
        }
    }
}

fn score_1(state: State, graph: &Graph) -> usize {
    if state.current.as_str() == "end" {
        return 1;
    }

    successors_1(&state, graph)
        .into_iter()
        .map(|s| score_1(s, graph))
        .sum()
}

fn can_move_to_1(n: &Node, visited: &HashSet<Node>) -> bool {
    n.chars().all(|x| x.is_uppercase()) || !visited.contains(n)
}

fn can_move_to_2(n: &Node, visited: &BTreeSet<Node>) -> bool {
    n.chars().all(|x| x.is_uppercase()) || !visited.contains(n)
}

fn successors_1(state: &State, graph: &Graph) -> SmallVec<[State; 6]> {
    graph
        .get(&state.current)
        .unwrap()
        .iter()
        .filter(|n| can_move_to_1(n, &state.little_visited))
        .cloned()
        .map(|n| {
            let mut visited = state.little_visited.clone();
            visited.insert(state.current.clone());
            State {
                current: n,
                little_visited: visited,
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash)]
pub struct State2 {
    current: Node,
    visited: BTreeSet<Node>,
    visited_small: bool,
}

impl State2 {
    pub fn start() -> Self {
        Self {
            current: Node::new("start"),
            visited: [Node::new("start")].into(),
            visited_small: false,
        }
    }
}

fn score_2(state: State2, graph: &Graph, memo: &mut HashMap<State2, usize>) -> usize {
    if let Some(n) = memo.get(&state) {
        return *n;
    }

    if state.current.as_str() == "end" {
        return 1;
    }

    let n = successors_2(&state, graph)
        .into_iter()
        .map(|s| score_2(s, graph, memo))
        .sum();

    memo.insert(state, n);

    n
}

fn successors_2(state: &State2, graph: &Graph) -> SmallVec<[State2; 6]> {
    let mut visited = state.visited.clone();
    visited.insert(state.current.clone());

    graph
        .get(&state.current)
        .unwrap()
        .iter()
        .filter(|n| n.as_str() != "start")
        .filter_map(|n| {
            if can_move_to_2(n, &visited) {
                Some(State2 {
                    current: n.clone(),
                    visited: visited.clone(),
                    visited_small: state.visited_small,
                })
            } else if !state.visited_small {
                Some(State2 {
                    current: n.clone(),
                    visited: visited.clone(),
                    visited_small: true,
                })
            } else {
                None
            }
        })
        .collect()
}
