use crate::day::CombinedSolver;
use std::collections::HashMap;
use std::io::BufRead;

pub struct Day1;

impl CombinedSolver for Day1 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(Result::unwrap)
            .map(|s| {
                let x = s.split_once("   ").unwrap();
                (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap())
            })
            .unzip();

        left.sort();
        right.sort();

        let part_1 = left
            .iter()
            .copied()
            .zip(right.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .sum::<u32>();

        let mut counts: HashMap<i32, i32> = HashMap::new();
        for i in right {
            *counts.entry(i).or_default() += 1;
        }

        let mut part_2 = 0;
        for i in left {
            part_2 += i * counts.get(&i).copied().unwrap_or_default();
        }

        Ok((part_1.to_string(), part_2.to_string()))
    }
}
