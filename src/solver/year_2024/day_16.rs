use glam::IVec2;
use hashbrown::HashSet;
use itertools::Itertools;
use pathfinding::prelude::{dijkstra, yen};
use smallvec::SmallVec;
use std::fmt::Display;
use std::io::BufRead;
use utils::direction::Direction;
use utils::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    pos: IVec2,
    dir: Direction,
}

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let (grid, end, start) = {
        let mut start_pos = IVec2::ZERO;
        let mut end_pos = IVec2::ZERO;
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => true,
                        '.' => false,
                        'S' => {
                            start_pos = IVec2::new(x as i32, y as i32);
                            false
                        }
                        'E' => {
                            end_pos = IVec2::new(x as i32, y as i32);
                            false
                        }
                        _ => panic!("{c}"),
                    })
                    .collect_vec()
            })
            .collect::<Grid<_>>();
        (
            grid,
            end_pos,
            State {
                pos: start_pos,
                dir: Direction::East,
            },
        )
    };

    let p_1 = {
        dijkstra(&start, |s| successors(s, &grid), |s| s.pos == end)
            .unwrap()
            .1
    };

    let p_2 = {
        let out = yen(&start, |s| successors(s, &grid), |s| s.pos == end, 10);
        let mut visited = HashSet::new();
        for (path, cost) in out {
            if cost != p_1 {
                break;
            }
            visited.extend(path.into_iter().map(|s| s.pos));
        }
        visited.len()
    };

    Ok((p_1, p_2))
}

fn successors(state: &State, grid: &Grid<bool>) -> SmallVec<[(State, usize); 3]> {
    let mut out = SmallVec::new();

    let forward = state.pos + state.dir.y_down();
    if !grid.get_i32(forward.x, forward.y).copied().unwrap_or(true) {
        out.push((
            State {
                pos: forward,
                dir: state.dir,
            },
            1,
        ));
    }

    out.push((
        State {
            pos: state.pos,
            dir: state.dir.cw(),
        },
        1000,
    ));

    out.push((
        State {
            pos: state.pos,
            dir: state.dir.ccw(),
        },
        1000,
    ));

    out
}
