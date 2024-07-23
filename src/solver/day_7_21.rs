use std::io::BufRead;
use itertools::Itertools;
use crate::day::{CombinedSolver, SeparatedSolver};

pub struct Day721;

impl CombinedSolver for Day721 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let line = input.lines().next().unwrap()?;
        let crabs = line.split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();

        fn solve(crabs: &[i32], mut cost: impl FnMut(i32, i32) -> i64) -> i64 {
            let max = crabs.iter().copied().max().unwrap();
            (0..=max)
                .map(|p| crabs.iter().map(|c| cost(p, *c)).sum())
                .min().unwrap()
        }

        #[inline]
        fn cost_11(p: i32, c: i32) -> i64 {
            (p - c).abs() as i64
        }

        #[inline]
        fn cost_22(p: i32, c: i32) -> i64 {
            let n = (p - c).abs() as i64;
            n * (n + 1) / 2
        }

        let part_1 = solve(crabs.as_slice(), cost_11);
        let part_2 = solve(crabs.as_slice(), cost_22);

        Ok((part_1.to_string(), part_2.to_string()))
    }
}