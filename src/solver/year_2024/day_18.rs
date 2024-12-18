use std::fmt::Display;
use std::io::BufRead;
use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::bfs;
use utils::grid::Grid;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let bytes = input.lines()
        .map(|l| { 
            let l = l.unwrap();
            let (a, b) = l.split_once(",").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .collect_vec();
    
    let (size, limit) = if bytes.len() == 25 { (7, 12) } else { (71, 1024) };
    
    
    let grid = {
        let mut grid = Grid::new(vec![vec![true; size]; size]);
        for (x, y) in bytes.iter().take(limit).copied() {
            // dbg!(x, y);
            *grid.get_i32_mut(x, y).unwrap() = false;
        }
        grid
    };
    
    let p_1 = bfs(
        &IVec2::ZERO,
        |s| {
            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .map(|p| p + *s)
                .into_iter()
                .filter(|x| grid.get_i32(x.x, x.y).is_some_and(|&x| x))
        },
        |s| s.x as usize == size - 1 && s.y as usize == size - 1
    ).unwrap().len() - 1;

    let p_2 = part_2(size, &bytes);
    
    Ok((p_1, format!("{},{}", p_2.0, p_2.1)))
}

fn part_2(size: usize, bytes: &[(i32, i32)]) -> (i32, i32) {
    
    let mut grid = Grid::new(vec![vec![true; size]; size]);
    let mut byte = bytes.iter().copied();
    loop {
        let (x, y) = byte.next().unwrap();
        *grid.get_i32_mut(x, y).unwrap() = false;
        if bfs(
            &IVec2::ZERO,
            |s| {
                [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                    .map(|p| p + *s)
                    .into_iter()
                    .filter(|x| grid.get_i32(x.x, x.y).is_some_and(|&x| x))
            },
            |s| s.x as usize == size - 1 && s.y as usize == size - 1
        ).is_none() {
            return (x, y);
        }
    }
}