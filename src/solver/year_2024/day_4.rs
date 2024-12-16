use crate::day::CombinedSolver;
use itertools::Itertools;
use std::io::BufRead;
use utils::grid::Grid;

pub struct Day4;

impl CombinedSolver for Day4 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let grid = input
            .lines()
            .map(Result::unwrap)
            .map(|l| l.chars().collect_vec())
            .collect::<Grid<char>>();

        let mut p_1 = 0;

        for ((x, y), &c) in grid.iter() {
            if c != 'X' {
                continue;
            }

            for [dx, dy] in [
                [1, 1],
                [1, 0],
                [1, -1],
                [0, -1],
                [0, 1],
                [-1, 1],
                [-1, 0],
                [-1, -1],
            ] {
                let x = x as i64;
                let y = y as i64;
                if grid.get_i(x + dx, y + dy) == Some(&'M')
                    && grid.get_i(x + 2 * dx, y + 2 * dy) == Some(&'A')
                    && grid.get_i(x + 3 * dx, y + 3 * dy) == Some(&'S')
                {
                    p_1 += 1;
                }
            }
        }

        let mut p_2 = 0;

        for y in 0..(grid.h - 2) {
            for x in 0..(grid.w - 2) {
                let a = *grid.get(x + 1, y + 1).unwrap();
                if a != 'A' {
                    continue;
                }
                let corners = [(x, y), (x + 2, y), (x + 2, y + 2), (x, y + 2)]
                    .map(|(x, y)| *grid.get(x, y).unwrap());

                if corners == ['M', 'M', 'S', 'S']
                    || corners == ['S', 'M', 'M', 'S']
                    || corners == ['S', 'S', 'M', 'M']
                    || corners == ['M', 'S', 'S', 'M']
                {
                    p_2 += 1;
                }
            }
        }

        Ok((p_1.to_string(), p_2.to_string()))
    }
}
