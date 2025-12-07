use hashbrown::{HashMap, HashSet};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use petgraph::data::Build;
use petgraph::prelude::NodeIndex;
use std::fmt::Display;
use std::io::BufRead;

type Node = [char; 2];
type Graph = petgraph::prelude::UnGraphMap<Node, u8>;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let mut graph = Graph::default();
    let mut indices = IndexSet::new();

    for line in input.lines() {
        let line = line?.chars().collect_vec();
        let a = [line[0], line[1]];
        let b = [line[3], line[4]];
        indices.insert(a);
        indices.insert(b);

        // let a = *indices.entry(a)
        //     .or_insert_with(|| graph.add_node(a));
        // let b = *indices.entry(b)
        //     .or_insert_with(|| graph.add_node(b));

        graph.add_edge(a, b, 1);
    }

    let mut p_1 = 0;
    for (a, b, c) in graph.nodes().tuple_combinations() {
        if ![a[0], b[0], c[0]].iter().any(|c| *c == 't') {
            continue;
        }
        if !graph.contains_edge(a, b) || !graph.contains_edge(b, c) || !graph.contains_edge(c, a) {
            continue;
        }
        p_1 += 1;
    }

    let mut best = vec![];
    let mut i = 2;
    loop {
        let (out, success) = find(vec![], 0, i, &graph, &indices);
        if !success {
            break;
        }
        best = out;
        i += 1;
    }
    best.sort();
    let p_2 = best
        .into_iter()
        .map(|c| format!("{}{}", c[0], c[1]))
        .join(",");

    Ok((p_1, p_2))
}

fn find(
    mut this: Vec<Node>,
    mut idx: usize,
    remaining: usize,
    graph: &Graph,
    nodes: &IndexSet<Node>,
) -> (Vec<Node>, bool) {
    if remaining == 0 {
        return (this, true);
    }

    while idx < nodes.len() {
        let b = *nodes.get_index(idx).unwrap();
        if !this.iter().all(|a| graph.contains_edge(*a, b)) {
            idx += 1;
            continue;
        }

        this.push(b);
        let success;
        (this, success) = find(this, idx + 1, remaining - 1, graph, nodes);
        if success {
            return (this, true);
        }
        this.pop();

        idx += 1;
    }

    (this, false)
}
