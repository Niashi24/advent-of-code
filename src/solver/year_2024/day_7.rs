use std::cmp::Ordering;
use std::io::BufRead;
use itertools::Itertools;
use utils::num_digits;
use crate::day::CombinedSolver;

pub struct Day7;

impl CombinedSolver for Day7 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        
        let mut p_1 = 0;
        let mut p_2 = 0;
        
        for (target, nums) in input.lines().map(Result::unwrap)
            .map(|s| {
                let (target, nums) = s.split_once(": ").unwrap();
                (target.parse::<u64>().unwrap(), nums.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect_vec())
            }) {
            if is_ok::<false>(1, nums[0], target, &nums) {
                p_1 += target;
            }
            if is_ok::<true>(1, nums[0], target, &nums) {
                p_2 += target;
            }
        }
        
        Ok((p_1.to_string(), p_2.to_string()))
    }
}


fn is_ok<const P2: bool>(index: usize, current_value: u64, target: u64, equations: &[u64]) -> bool {
    match current_value.cmp(&target) {
        Ordering::Equal if index == equations.len() => true, // are we at the end?
        Ordering::Greater => false,
        Ordering::Less if index == equations.len() => false,
        _ => is_ok::<P2>(index + 1, current_value * equations[index], target, equations) ||
            is_ok::<P2>(index + 1, current_value + equations[index], target, equations) ||
            (P2 && is_ok::<P2>(index + 1, concat(current_value, equations[index]), target, equations))
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(num_digits(b)) + b
}