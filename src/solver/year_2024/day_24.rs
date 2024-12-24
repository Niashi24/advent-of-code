use hashbrown::HashMap;
use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::prelude::DiGraphMap;
use regex::Regex;
use std::fmt::Display;
use std::io::BufRead;
use std::str::FromStr;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let (mut known, graph) = parse(input);
    for node in toposort(&graph, None).unwrap().into_iter().rev() {
        if known.contains_key(&node) {
            continue;
        }
        
        let mut edges = graph.edges(node);
        
        let (_, a, op) = edges.next().unwrap();
        let (_, b, _) = edges.next().unwrap();
        let a = *known.get(&a).unwrap();
        let b = *known.get(&b).unwrap();
        
        known.insert(node, op.apply(a, b));
    }
    
    let mut out = 0;
    for (i, &x) in (0..)
        .map(|i| str_to_id(&format!("z{i:02}")).unwrap())
        .map(|id| known.get(&id))
        .take_while(|x| x.is_some())
        .map(|x| x.unwrap())
        .enumerate() {
        out += (x as u64) << i;
    }
    
    Ok(out)
}

type NodeId = [char; 3];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
enum Operation {
    And,
    Xor,
    Or,
}

impl Operation {
    pub fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Operation::And => a && b,
            Operation::Xor => a ^ b,
            Operation::Or => a || b,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OR" => Ok(Operation::Or),
            "AND" => Ok(Operation::And),
            "XOR" => Ok(Operation::Xor),
            _ => Err(s.to_string()),
        }
    }
}

type Graph = DiGraphMap<NodeId, Operation>;

pub fn parse(input: Box<dyn BufRead>) -> (HashMap<NodeId, bool>, Graph) {
    let mut lines = input.lines().map(Result::unwrap).peekable();
    
    let inputs = lines.peeking_take_while(|x| !x.is_empty())
        .map(|s| {
            let (node, value) = s.split_once(": ").unwrap();
            let node = node.chars().collect_vec().try_into().unwrap();
            let value = match value {
                "0" => false,
                "1" => true,
                _ => panic!("{value}"),
            };

            (node, value)
        })
        .collect();
    
    lines.next();
    
    let mut graph = Graph::new();
    let regex = Regex::new(r#"(?<in1>...) (?<op>AND|OR|XOR) (?<in2>...) -> (?<out>...)"#).unwrap();
    
    for line in lines {
        let captures = regex.captures(&line).unwrap();
        let a = str_to_id(&captures["in1"]).unwrap();
        let b = str_to_id(&captures["in2"]).unwrap();
        let out = str_to_id(&captures["out"]).unwrap();
        let op = captures["op"].parse().unwrap();
        graph.add_edge(out, a, op);
        graph.add_edge(out, b, op);
    }

    (inputs, graph)
}

fn str_to_id(s: &str) -> Option<[char; 3]> {
    s.chars()
        .collect_vec()
        .try_into()
        .ok()
}
