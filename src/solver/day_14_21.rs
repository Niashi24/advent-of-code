use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::BufRead;
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day1421;

impl CombinedSolver for Day1421 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let template = lines.next().unwrap();
        lines.next();

        let rules = lines.map(|l| {
            let out = l.chars().rev().next().unwrap();
            let pair = l.chars().next_tuple().unwrap();
            Rule {
                pair,
                out,
            }
        }).collect_vec();

        let mut part_1_polymer = template.clone();
        for i in 0..20 {
            part_1_polymer = step(part_1_polymer, &rules);
            let mut counts = part_1_polymer.chars().counts().into_iter()
                .sorted_by_key(|a| a.1);
            let least = counts.next().unwrap().1;
            let most = counts.last().unwrap().1;
            println!("{i} {least} {most}");
        }

        let mut counts = part_1_polymer.chars().counts().into_iter()
            .sorted_by_key(|a| a.1);
        let least = counts.next().unwrap().1;
        let most = counts.last().unwrap().1;
        let part_1 = most - least;

        Ok((part_1.to_string(), Default::default()))
    }
}

fn step(s: String, rules: &[Rule]) -> String {
    let mut insertions = BTreeMap::new();

    rules.iter().flat_map(|r| r.matches(&s))
        .for_each(|(c, i)| { insertions.insert(i, c); });

    // println!("{:?}", insertions);

    if insertions.is_empty() {
        return s;
    }

    let mut chars = s.chars();
    let mut s = String::new();

    let (mut i, c) = insertions.pop_first().unwrap();
    for _ in 0..i {
        s.push(chars.next().unwrap());
    }
    s.push(c);

    for (j, c) in insertions {
        for _ in 0..(j - i) {
            s.push(chars.next().unwrap());
        }
        s.push(c);
        i = j;
    }
    s.extend(chars);

    s
}

struct Rule {
    pair: (char, char),
    out: char,
}

impl Rule {
    fn matches(&self, polymer: &str) -> Vec<(char, usize)> {
        polymer.chars().tuple_windows::<(_,_)>()
            .enumerate()
            .filter(|&(_, p)| p == self.pair)
            .map(|(i, _)| (self.out, i + 1))
            .collect()
    }
}


