use std::collections::BinaryHeap;
use std::io::BufRead;

use indexmap::IndexSet;
use itertools::Itertools;
use pathfinding::prelude::bfs_reach;
use smallvec::SmallVec;

use crate::day::CombinedSolver;
use crate::grid::Grid;

pub struct Day9;

impl CombinedSolver for Day9 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let grid: Grid<u8> = input
            .lines()
            .map(Result::unwrap)
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect();

        let low_points = grid
            .iter()
            .filter(|&((x, y), a)| {
                // top
                successors((x, y), &grid)
                    .into_iter()
                    .map(|(x, y)| *grid.get(x, y).unwrap())
                    .all(|b| *a < b)
            })
            .map(|(p, _)| p)
            .collect_vec();

        let part_1 = low_points
            .iter()
            .copied()
            .map(|(x, y)| *grid.get(x, y).unwrap() as usize + 1)
            .sum::<usize>();

        let mut to_visit: IndexSet<_> = low_points.into_iter().collect();

        let mut total = BinaryHeap::new();
        while let Some(start) = to_visit.pop() {
            let mut count = 0;
            for x in bfs_reach(start, |&p| {
                successors(p, &grid)
                    .into_iter()
                    .filter(|&(x, y)| !grid.get(x, y).is_some_and(|n| *n == 9))
            }) {
                to_visit.swap_remove(&x);
                count += 1;
            }
            total.push(count);
        }

        let (Some(a), Some(b), Some(c)) = (total.pop(), total.pop(), total.pop()) else {
            panic!("less than 3 basins")
        };

        let part_2 = a * b * c;

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

fn successors((x, y): (usize, usize), grid: &Grid<u8>) -> SmallVec<[(usize, usize); 4]> {
    let mut vec = SmallVec::new();
    // if let Some(n) = x.checked_sub(1).and_then(|x| grid.get(x, y))
    if x != 0 {
        vec.push((x - 1, y));
    }
    if y != 0 {
        vec.push((x, y - 1));
    }
    if x + 1 != grid.w {
        vec.push((x + 1, y));
    }
    if y + 1 != grid.h {
        vec.push((x, y + 1));
    }

    vec
}
