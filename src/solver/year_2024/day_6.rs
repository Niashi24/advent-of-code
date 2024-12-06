use std::collections::HashSet;
use std::io::BufRead;
use itertools::Itertools;
use utils::grid::Grid;
use crate::day::CombinedSolver;

pub struct Day6;

impl CombinedSolver for Day6 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut initial_position = (0, 0);
        
        let mut grid = input.lines().map(Result::unwrap)
            .enumerate()
            .map(|(y, l)| l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => false,
                    '#' => true,
                    '^' => {
                        initial_position = (x as i64, y as i64);
                        false
                    }
                    _ => panic!("{c}")
                }).collect_vec()
            )
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
        
        let mut p_2 = 0;
        for y in 0..grid.w {
            for x in 0..grid.h {
                let pos = grid.get_mut(x, y).unwrap();
                
                if *pos || (x as i64, y as i64) == initial_position {
                    continue;
                }
                
                *pos = true;
                
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
                
                *grid.get_mut(x, y).unwrap() = false;
            }
        }
        
        Ok((visited.len().to_string(), p_2.to_string()))
    }
}