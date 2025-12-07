use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::{bfs, bfs_reach};
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::io::BufRead;
use utils::grid::Grid;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let (grid, start, end) = {
        let mut start = None;
        let mut end = None;
        let grid: Grid<bool> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start = Some(IVec2::new(x as i32, y as i32));
                            true
                        }
                        'E' => {
                            end = Some(IVec2::new(x as i32, y as i32));
                            true
                        }
                        _ => panic!("{c}"),
                    })
                    .collect_vec()
            })
            .collect();
        (grid, start.unwrap(), end.unwrap())
    };

    let path = bfs(
        &start,
        |&p| {
            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .into_iter()
                .map(move |s| s + p)
                .filter(|s| grid.get_i32(s.x, s.y).copied().unwrap_or_default())
        },
        |p| *p == end,
    )
    .unwrap();

    let mut n_grid: Grid<Option<usize>> = Grid::new(vec![vec![None; grid.w]; grid.h]);
    for (n, &p) in path.iter().enumerate() {
        *n_grid.get_i32_mut(p.x, p.y).unwrap() = Some(n);
    }

    let p_1 = {
        let cutoff = if grid.w == 15 { 20 } else { 100 };
        let mut num_over = 0;

        for &p in &path {
            let this = n_grid.get_i32(p.x, p.y).unwrap().unwrap() as i32;
            // println!("{:?}: {this}", p);

            num_over += [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .map(|s| s * 2 + p)
                .into_iter()
                .filter_map(|x| n_grid.get_i32(x.x, x.y).copied().flatten())
                .map(|i| i as i32 - this - 2)
                .filter(|i| (*i) >= cutoff)
                // .map(|x| dbg!(x))
                .count();
        }
        num_over
    };

    let p_2 = {
        let cutoff = if grid.w == 15 { 50 } else { 100 };
        let mut num_over = 0;

        for &pos in &path {
            let this = n_grid.get_i32(pos.x, pos.y).unwrap().unwrap() as i32;

            let mut to_visit = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .map(|s| s + pos)
                .into_iter()
                // .filter(|p| grid.get_i32(p.x, p.y).is_some_and(|b| !*b))
                .map(|p| (p, 1))
                .collect::<VecDeque<_>>();
            let mut visited = HashSet::new();

            while let Some((p, c)) = to_visit.pop_front() {
                if c > 20 || !visited.insert(p) {
                    continue;
                }

                match n_grid.get_i32(p.x, p.y).copied() {
                    None => continue,
                    Some(None) => {
                        to_visit.extend(
                            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                                .map(|s| s + p)
                                .into_iter()
                                .filter(|p| !visited.contains(p))
                                .map(|p| (p, c + 1)),
                        );
                    }
                    Some(Some(x)) => {
                        to_visit.extend(
                            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                                .map(|s| s + p)
                                .into_iter()
                                .filter(|p| !visited.contains(p))
                                .map(|p| (p, c + 1)),
                        );

                        // if (x as i32) < this {
                        //     continue;
                        // }
                        let c = p.x.abs_diff(pos.x) as i32 + p.y.abs_diff(pos.y) as i32;
                        let saved = (x as i32 - this) - c;
                        // println!("{pos} -> {p}: {saved}, {c} {this} {x}");
                        if saved >= cutoff {
                            num_over += 1;
                        }
                    }
                }
            }
        }

        num_over
    };

    Ok((p_1, p_2))
}
