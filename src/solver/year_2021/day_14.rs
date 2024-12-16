use crate::day::CombinedSolver;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::io::BufRead;
use std::rc::Rc;

pub struct Day14;

impl CombinedSolver for Day14 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let template: Vec<u8> = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect();
        lines.next();

        let rules = lines
            .map(|l| {
                let out = l.chars().next_back().unwrap().try_into().unwrap();
                let pair = l
                    .chars()
                    .map(|c| c.try_into().unwrap())
                    .next_tuple()
                    .unwrap();
                Rule { pair, out }
            })
            .collect_vec();

        let mut memo = Memo::new();

        let part_1 = solve(&template, &rules, 10, &mut memo);
        let part_2 = solve(&template, &rules, 40, &mut memo);

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

pub type Counts = BTreeMap<u8, usize>;

fn solve(initial: &[u8], rules: &[Rule], t: u8, memo: &mut Memo) -> usize {
    let mut counts = initial.iter().copied().counts().into_iter().collect();
    for pair in initial.iter().copied().tuple_windows::<(_, _)>() {
        if let Some(rule) = rules.iter().find(|r| r.matches(pair)) {
            let c = score(pair.0, pair.1, rule.out, t, rules, memo);
            add_all(&mut counts, &c);
        }
    }

    let (min, max) = counts.into_values().minmax().into_option().unwrap();

    max - min
}

type Memo = HashMap<(u8, u8, u8, u8), Rc<Counts>>;

fn add_all(a: &mut Counts, b: &Counts) {
    for (i, n) in b {
        *a.entry(*i).or_default() += n;
    }
}

fn score(a: u8, b: u8, i: u8, t: u8, rules: &[Rule], memo: &mut Memo) -> Rc<Counts> {
    if let Some(counts) = memo.get(&(a, b, i, t)) {
        return counts.clone();
    }

    let mut counts = Counts::from([(i, 1)]);
    if t == 1 {
        return Rc::new(counts);
    }

    rules
        .iter()
        .filter(|r| r.matches((a, i)))
        .map(|r| score(a, i, r.out, t - 1, rules, memo))
        .for_each(|c| add_all(&mut counts, &c));

    rules
        .iter()
        .filter(|r| r.matches((i, b)))
        .map(|r| score(i, b, r.out, t - 1, rules, memo))
        .for_each(|c| add_all(&mut counts, &c));

    let counts = Rc::new(counts);
    memo.insert((a, b, i, t), counts.clone());

    counts
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    pair: (u8, u8),
    out: u8,
}

impl Rule {
    fn matches(&self, p: (u8, u8)) -> bool {
        self.pair == p
    }
}
