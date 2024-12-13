use glam::IVec2;
use itertools::Itertools;
use smallset::SmallSet;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::BufRead;
use utils::grid::Grid;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let grid: Grid<u8> = input.lines()
        .map(|l| l.unwrap().chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect_vec())
        .collect();
    
    let mut p_1 = 0;
    let mut p_2 = 0;
    let mut memo_1 = HashMap::new();
    let mut memo_2 = HashMap::new();
    for ((x, y), &v) in grid.iter() {
        if v != 0 {
            continue;
        }
        
        // p_1 += find_1(IVec2::new(x as i32, y as i32), &grid);
        
        p_1 += find(IVec2::new(x as i32, y as i32), &grid, &mut memo_1).len();
        p_2 += find_2(IVec2::new(x as i32, y as i32), &grid, &mut memo_2);
    }
    
    Ok((p_1, p_2))
}

type Storage = SmallSet<[IVec2; 6]>;

fn find(pos: IVec2, grid: &Grid<u8>, memo: &mut HashMap<IVec2, Storage>) -> Storage {
    if let Some(n) = memo.get(&pos) {
        return n.clone();
    }

    let cur = *grid.get_i32(pos.x, pos.y).unwrap();
    if cur == 9 {
        memo.insert(pos, Storage::from_iter([pos]));
        
        return memo.get(&pos).unwrap().clone();
    }
    
    let mut out = Storage::new();
    
    for p in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y].into_iter()
        .map(|p| p + pos)
        .filter(|p| grid.get_i32(p.x, p.y).copied()
            .is_some_and(|v| (v) == cur + 1)) {
        
        for &p in find(p, grid, memo).iter() {
            out.insert(p);
        }
    }
    
    memo.insert(pos, out);

    memo.get(&pos).unwrap().clone()
}

fn find_2(pos: IVec2, grid: &Grid<u8>, memo: &mut HashMap<IVec2, usize>) -> usize {
    if let Some(&n) = memo.get(&pos) {
        return n;
    }

    let cur = *grid.get_i32(pos.x, pos.y).unwrap();
    if cur == 9 {
        return 1;
    }

    let mut out = 0;

    for p in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y].into_iter()
        .map(|p| p + pos)
        .filter(|p| grid.get_i32(p.x, p.y).copied()
            .is_some_and(|v| (v) == cur + 1)) {
        
        out += find_2(p, grid, memo);
    }

    memo.insert(pos, out);


    *memo.get(&pos).unwrap()
}

// intended solution
// only slightly faster because the other one can be shared between runs
fn _find_1(pos: IVec2, grid: &Grid<u8>) -> usize {
    let mut count = 0;
    let mut visited = HashSet::new();

    let mut to_visit = vec![pos];
    while let Some(pos) = to_visit.pop() {
        if !visited.insert(pos) {
            continue;
        }

        let cur = *grid.get_i32(pos.x, pos.y).unwrap();
        if cur == 9 {
            count += 1;
            continue;
        }

        to_visit.extend([IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y].into_iter()
            .map(|p| p + pos)
            .filter(|p| grid.get_i32(p.x, p.y).copied()
                .is_some_and(|v| (v) == cur + 1)));
    }

    count
}
