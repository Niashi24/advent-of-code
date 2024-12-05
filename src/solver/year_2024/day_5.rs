use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day5;

impl CombinedSolver for Day5 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        // key must be after values
        let mut orderings = HashMap::<i64, HashSet<i64>>::new();
        
        let mut input = input.lines().map(Result::unwrap).peekable();
        
        for (a, b) in input.peeking_take_while(|s| !s.is_empty())
            .map(|s| { 
                let (a, b) = s.split_once("|").unwrap();
                (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
            }) {
            
            orderings.entry(b).or_default().insert(a);
        }
        
        // skip empty line
        input.next();
        
        let updates = input
            .map(|l| l.split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec());
        
        let mut p_1 = 0;
        let mut p_2 = 0;
        
        for mut update in updates {
            if ordered(&update, &orderings) {
                p_1 += update[update.len() / 2];
            } else {
                update.sort_unstable_by(|a, b| {
                    if orderings.get(a).is_some_and(|s| s.contains(b)) {
                        Ordering::Less
                    } else if orderings.get(b).is_some_and(|s| s.contains(a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                
                p_2 += update[update.len() / 2];
            }
        }
        
        Ok((p_1.to_string(), p_2.to_string()))
    }
}

fn ordered(update: &[i64], bad_orderings: &HashMap<i64, HashSet<i64>>) -> bool {
    for i in 0..(update.len() - 1) {
        let a = update[i];
        let Some(bad) = bad_orderings.get(&a) else {
            continue;
        };
        
        for b in &update[i+1..] {
            if bad.contains(b) {
                return false;
            }
        }
    }
    
    true
}