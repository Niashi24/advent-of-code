use crate::day::{CombinedSolver, SeparatedSolver};
use std::io::BufRead;
use itertools::Itertools;

pub struct Day121;

impl SeparatedSolver for Day121 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {

        let mut count: usize = 0;
        for (a, b) in input.lines()
            .map(|l| l.unwrap().parse::<i32>().unwrap())
            .tuple_windows() {

            if b > a {
                count += 1;
            }
        }
        Ok(count.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let mut count: usize = 0;
        for (a, b, c, d) in input.lines()
            .map(|l| l.unwrap().parse::<i32>().unwrap())
            .tuple_windows() {

            if (a + b + c) < (b + c + d) {
                count += 1;
            }
        }
        Ok(count.to_string())
    }
}
