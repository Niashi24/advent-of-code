use crate::day::CombinedSolver;
use memoize::memoize;
use std::io::BufRead;

pub struct Day6;

impl CombinedSolver for Day6 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let (p_1, p_2) = input
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .map(|n| (total(n, 80), total(n, 256)))
            .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));

        Ok((p_1.to_string(), p_2.to_string()))
    }
}

#[memoize]
fn total(i: i32, mut days: i32) -> i64 {
    let mut count = 1;

    days -= i;

    while days > 0 {
        count += total(9, days);
        days -= 7;
    }

    count
}
