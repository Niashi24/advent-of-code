use std::io::BufRead;

use itertools::Itertools;
use utils::grid::Grid;


pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(usize, usize)> {
    let mut grid: Grid<bool> = input.lines().map(Result::unwrap)
        .map(|s: String| {
            s.chars().map(|s| s == '@').collect_vec()
        })
        .collect();

    let to_search = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut rolls = 0;

    for y in 0..grid.h as i32 {
        for x in 0..grid.w as i32 {

            if !grid.get_i32(x, y).unwrap() { continue; }

            if to_search.into_iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|&(x, y)| grid.get_i32(x, y).copied().unwrap_or(false))
                .count() < 4 {
                rolls += 1;
            }

        }
    }

    let mut rolls_2 = 0;
    
    loop {
        let mut any = false;
        for y in 0..grid.h as i32 {
            for x in 0..grid.w as i32 {

                if !grid.get_i32(x, y).unwrap() { continue; }

                if to_search.into_iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|&(x, y)| grid.get_i32(x, y).copied().unwrap_or(false))
                    .count() < 4 {
                    rolls_2 += 1;
                    any = true;
                    *grid.get_i32_mut(x, y).unwrap() = false;
                }

            }
        }

        if !any { break; }
    }

    Ok((rolls, rolls_2))
}