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
            if make_ordered(&mut update, &orderings) {
                p_2 += update[update.len() / 2];
            } else {
                p_1 += update[update.len() / 2];
            }
        }
        
        Ok((p_1.to_string(), p_2.to_string()))
    }
}

fn make_ordered(update: &mut [i64], bad_orderings: &HashMap<i64, HashSet<i64>>) -> bool {
    let mut made_changes = false;
    while let Err((a, b)) = ordered(update, bad_orderings) {
        update.swap(a, b);
        made_changes = true;
    }
    made_changes
}

fn ordered(update: &[i64], bad_orderings: &HashMap<i64, HashSet<i64>>) -> Result<(), (usize, usize)> {
    for i in 0..(update.len() - 1) {
        let a = update[i];
        let Some(bad) = bad_orderings.get(&a) else {
            continue;
        };
        
        for (j, b) in update.iter().enumerate().skip(i + 1) {
            if bad.contains(b) {
                return Err((i, j));
            }
        }
    }
    
    Ok(())
}