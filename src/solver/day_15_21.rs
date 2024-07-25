use std::io::BufRead;
use itertools::Itertools;
use pathfinding::prelude::{astar, bfs, dijkstra};
use smallvec::SmallVec;
use crate::day::CombinedSolver;
use crate::grid::Grid;

pub struct Day1521;

impl CombinedSolver for Day1521 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut grid: Grid<u8> = input.lines().map(Result::unwrap)
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect_vec())
            .collect();

        let bottom_right = (grid.w - 1, grid.h - 1);

        let risk = dijkstra(
            &(0, 0),
            |p| successors(*p, grid.w, grid.h).into_iter().map(|p| (p, *grid.get(p.0, p.1).unwrap() as usize)),
            |p| *p == bottom_right,
        ).unwrap().1;

        let bottom_right = (grid.w * 5 - 1, grid.h * 5 - 1);

        let risk_2 = astar(
            &(0, 0),
            |p| successors(*p, grid.w * 5, grid.h * 5).into_iter()
                .map(|p| (p, cost(p, &grid))),
            |p| bottom_right.0.abs_diff(p.0) + bottom_right.1.abs_diff(p.1),
            |p| *p == bottom_right,
        ).unwrap().1;

        Ok((risk.to_string(), risk_2.to_string()))
    }
}

fn cost((x, y): (usize, usize), grid: &Grid<u8>) -> usize {
    let (w_x, t_x) = (x / grid.w, x % grid.w);
    let (w_y, t_y) = (y / grid.h, y % grid.h);
    let mut cost = (*grid.get(t_x, t_y).unwrap() as usize + w_x + w_y);
    while cost > 9 {
        cost -= 9;
    }

    cost
}

fn successors((x, y): (usize, usize), w: usize, h: usize) -> SmallVec<[(usize, usize); 4]> {
    let mut out = SmallVec::new();

    if x != 0 {
        out.push((x - 1, y));
    }
    if y != 0 {
        out.push((x, y - 1));
    }
    if x + 1 != w {
        out.push((x + 1, y));
    }
    if y + 1 != h {
        out.push((x, y + 1));
    }

    out
}