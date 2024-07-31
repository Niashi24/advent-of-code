use crate::day::SeparatedSolver;
use crate::grid::Grid;
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::HashSet;
use std::io::BufRead;

pub struct Day11;

impl SeparatedSolver for Day11 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let mut grid: Grid<u8> = input
            .lines()
            .map(Result::unwrap)
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect();

        let mut count = 0;
        for _ in 0..100 {
            let flashed;
            (grid, flashed) = step(grid);
            count += flashed;
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let mut grid: Grid<u8> = input
            .lines()
            .map(Result::unwrap)
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect();

        let mut n = 0;
        for i in 1.. {
            let flashed;
            (grid, flashed) = step(grid);

            if flashed == 100 {
                n = i;
                break;
            }
        }

        Ok(n.to_string())
    }
}

fn step(mut grid: Grid<u8>) -> (Grid<u8>, usize) {
    grid.iter_mut().for_each(|(_, i)| *i += 1);

    let mut to_flash = grid
        .iter_mut()
        .filter(|(_, t)| **t > 9)
        .map(|(p, _)| p)
        .collect_vec();

    let mut flashing: HashSet<_> = to_flash.iter().copied().collect();

    while let Some((x, y)) = to_flash.pop() {
        for (x, y) in successors((x, y), &grid) {
            if flashing.contains(&(x, y)) {
                continue;
            }
            let p = grid.get_mut(x, y).unwrap();
            *p += 1;
            if *p > 9 {
                flashing.insert((x, y));
                to_flash.push((x, y));
            }
        }
    }

    for &(x, y) in &flashing {
        *grid.get_mut(x, y).unwrap() = 0;
    }

    (grid, flashing.len())
}

fn successors<T>((x, y): (usize, usize), grid: &Grid<T>) -> SmallVec<[(usize, usize); 9]> {
    let mut x_s = SmallVec::<[usize; 3]>::new();
    x_s.push(x);
    let mut y_s = SmallVec::<[usize; 3]>::new();
    y_s.push(y);

    if x != 0 {
        x_s.push(x - 1);
    }
    if y != 0 {
        y_s.push(y - 1);
    }
    if x + 1 != grid.w {
        x_s.push(x + 1);
    }
    if y + 1 != grid.h {
        y_s.push(y + 1);
    }

    let mut out: SmallVec<[(usize, usize); 9]> = x_s.into_iter().cartesian_product(y_s).collect();
    out.swap_remove(0); // first one == (x, y)

    out
}
