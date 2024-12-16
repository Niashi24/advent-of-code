use crate::day::CombinedSolver;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;
use utils::grid::Grid;

pub struct Day6;

impl CombinedSolver for Day6 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut initial_position = (0, 0);

        let mut grid = input
            .lines()
            .map(Result::unwrap)
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => false,
                        '#' => true,
                        '^' => {
                            initial_position = (x as i64, y as i64);
                            false
                        }
                        _ => panic!("{c}"),
                    })
                    .collect_vec()
            })
            .collect::<Grid<bool>>();

        let mut position = initial_position;
        let mut direction = (0, -1);
        let mut visited = HashSet::new();

        loop {
            visited.insert(position);

            let new_pos = (position.0 + direction.0, position.1 + direction.1);
            match grid.get_i(new_pos.0, new_pos.1) {
                None => break,
                Some(false) => position = new_pos,
                Some(true) => direction = (-direction.1, direction.0),
            }
        }
        let p_1 = visited.len();

        let mut p_2 = 0;
        for (x, y) in visited {
            if (x, y) == initial_position {
                continue;
            }

            *grid.get_mut(x as usize, y as usize).unwrap() = true;

            let mut visited = HashSet::new();

            let mut position = initial_position;
            let mut direction = (0, -1);

            loop {
                if !visited.insert((position, direction)) {
                    p_2 += 1;
                    break;
                }

                let new_pos = (position.0 + direction.0, position.1 + direction.1);
                match grid.get_i(new_pos.0, new_pos.1) {
                    None => break,
                    Some(false) => position = new_pos,
                    Some(true) => direction = (-direction.1, direction.0),
                }
            }

            *grid.get_mut(x as usize, y as usize).unwrap() = false;
        }

        Ok((p_1.to_string(), p_2.to_string()))
    }
}
