use crate::day::CombinedSolver;
use glam::I64Vec2;
use itertools::Itertools;
use std::io::BufRead;
use utils::grid::Grid;

pub struct Day8;

impl CombinedSolver for Day8 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let grid = input
            .lines()
            .map(Result::unwrap)
            .map(|l| l.chars().collect_vec())
            .collect::<Grid<char>>();

        let mut antinodes = Grid::new(vec![vec![false; grid.w]; grid.h]);

        for ((p_1, &a), (p_2, &b)) in grid
            .iter()
            .filter(|(_, c)| **c != '.')
            .map(|((x, y), c)| (I64Vec2::new(x as i64, y as i64), c))
            .tuple_combinations()
        {
            if a == b {
                let d_p = p_2 - p_1;
                if let Some(x) = antinodes.get_i_mut(p_2.x + d_p.x, p_2.y + d_p.y) {
                    *x = true;
                }
                if let Some(x) = antinodes.get_i_mut(p_1.x - d_p.x, p_1.y - d_p.y) {
                    *x = true;
                }
            }
        }

        let p_1 = antinodes.iter().filter(|(_, x)| **x).count();

        let mut antinodes = Grid::new(vec![vec![false; grid.w]; grid.h]);

        for ((p_1, &a), (p_2, &b)) in grid
            .iter()
            .filter(|(_, c)| **c != '.')
            .map(|((x, y), c)| (I64Vec2::new(x as i64, y as i64), c))
            .tuple_combinations()
        {
            if a == b {
                let d_p = p_2 - p_1;
                {
                    let mut cur = p_1;
                    while let Some(x) = antinodes.get_i_mut(cur.x, cur.y) {
                        *x = true;
                        cur -= d_p;
                    }
                }
                {
                    let mut cur = p_2;
                    while let Some(x) = antinodes.get_i_mut(cur.x, cur.y) {
                        *x = true;
                        cur += d_p;
                    }
                }
            }
        }

        let p_2 = antinodes.iter().filter(|(_, x)| **x).count();

        Ok((p_1.to_string(), p_2.to_string()))
    }
}
