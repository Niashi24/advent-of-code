use std::io::BufRead;
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day2;

impl CombinedSolver for Day2 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let segments = input.lines().map(Result::unwrap)
            .map(|s| s.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap()).collect_vec()).collect_vec();
        
        let p_1 = segments.iter().filter(|x| is_safe(x)).count();

        let p_2 = segments.into_iter().map(is_safe_2).filter(|x| *x).count();
        
        Ok((p_1.to_string(), p_2.to_string()))
    }
}

fn is_safe_2(mut levels: Vec<i32>) -> bool {
    if is_safe(&levels) {
        return true;
    }
    
    for i in 0..levels.len() {
        let removed = levels.remove(i);
        
        if is_safe(&levels) {
            return true;
        }
        
        levels.insert(i, removed);
    }
    
    false
}

fn is_safe(levels: &[i32]) -> bool {
    let ordering = levels[0].cmp(&levels[1]);
    
    for (a, b) in levels.iter().copied().tuple_windows() {
        if a.cmp(&b) != ordering {
            return false;
        }
        
        if !(1..=3).contains(&a.abs_diff(b)) {
            return false;
        }
    }
    
    true
}