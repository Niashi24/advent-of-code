use glam::IVec2;
use hashbrown::HashSet;
use indexmap::IndexSet;
use itertools::Itertools;
use std::fmt::Display;
use std::io::BufRead;
use utils::grid::Grid;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let grid: Grid<char> = input
        .lines()
        .map(|l| l.unwrap().chars().collect_vec())
        .collect();

    let mut visited = HashSet::new();
    let mut p_1 = 0;
    let mut p_2 = 0;
    for ((x, y), &c) in grid.iter() {
        let pos = IVec2::new(x as i32, y as i32);
        if visited.contains(&pos) {
            continue;
        }

        let mut region = Region::new(c);

        // flood fill
        let mut to_visit = vec![pos];
        while let Some(p) = to_visit.pop() {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);
            region.push(p);

            for next in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y].map(|x| x + p) {
                if grid.get_i32(next.x, next.y).is_some_and(|x| c == *x) {
                    to_visit.push(next);
                }
            }
        }

        // create walls
        let mut walls = IndexSet::new(); // I don't really need an index set, just want a .pop() method lol
        for &p in &region.positions {
            for d in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
                let dp = p + d;
                if grid.get_i32(dp.x, dp.y).is_none_or(|c| region.c != *c) {
                    walls.insert(Wall {
                        c: region.c,
                        center: p,
                        normal: d,
                    });
                }
            }
        }

        // simple perimeter
        p_1 += region.positions.len() * walls.len();

        // merge walls on perimeter
        let mut count = 0;
        while let Some(wall) = walls.pop() {
            count += 1;
            // flood fill and remove all on this wall
            let mut to_visit = vec![wall];
            while let Some(wall) = to_visit.pop() {
                let left = wall.left();
                if walls.swap_remove(&(left)) {
                    to_visit.push(left);
                }
                let right = wall.right();
                if walls.swap_remove(&(right)) {
                    to_visit.push(right);
                }
            }
        }

        p_2 += region.positions.len() * count;
    }

    Ok((p_1, p_2))
}

struct Region {
    c: char,
    positions: Vec<IVec2>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Wall {
    c: char,
    center: IVec2,
    normal: IVec2,
}

impl Wall {
    fn left(self) -> Self {
        Self {
            center: self.center + IVec2::new(self.normal.y, -self.normal.x),
            ..self
        }
    }

    fn right(self) -> Self {
        Self {
            center: self.center + IVec2::new(-self.normal.y, self.normal.x),
            ..self
        }
    }
}

impl Region {
    pub fn new(c: char) -> Self {
        Self {
            c,
            positions: vec![],
        }
    }

    pub fn push(&mut self, p: IVec2) {
        self.positions.push(p);
    }
}
