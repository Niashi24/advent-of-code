use crate::day::CombinedSolver;
use itertools::{Either, Itertools};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use utils::grid::Grid;

pub struct Day25;

impl CombinedSolver for Day25 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let grid: Grid<Cell> = input
            .lines()
            .map(Result::unwrap)
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'v' => Cell(Some(Dir::South)),
                        '>' => Cell(Some(Dir::East)),
                        '.' => Cell(None),
                        _ => panic!("{c}"),
                    })
                    .collect_vec()
            })
            .collect();

        let part_1 = part_1(grid.clone());

        Ok((part_1.to_string(), "".to_string()))
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    East,
    South,
}

impl Dir {
    fn is_east(&self) -> bool {
        match self {
            Dir::East => true,
            Dir::South => false,
        }
    }

    fn is_south(&self) -> bool {
        match self {
            Dir::East => false,
            Dir::South => true,
        }
    }

    fn step(&self, (x, y): (usize, usize), grid: &Grid<Cell>) -> (usize, usize) {
        match self {
            Dir::East => {
                if x + 1 == grid.w {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            }
            Dir::South => {
                if y + 1 == grid.h {
                    (x, 0)
                } else {
                    (x, y + 1)
                }
            }
        }
    }

    fn unstep(&self, (x, y): (usize, usize), grid: &Grid<Cell>) -> (usize, usize) {
        match self {
            Dir::East => {
                if x == 0 {
                    (grid.w - 1, y)
                } else {
                    (x - 1, y)
                }
            }
            Dir::South => {
                if y == 0 {
                    (x, grid.h - 1)
                } else {
                    (x, y - 1)
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Cell(Option<Dir>);

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Some(Dir::East) => '>',
                Some(Dir::South) => 'v',
                None => '.',
            }
        )
    }
}

fn part_1(mut grid: Grid<Cell>) -> usize {
    // #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    // struct State((u64, u64), usize);
    //
    // impl Ord for State {
    //     fn cmp(&self, other: &Self) -> Ordering {
    //         self.1.cmp(&other.1)
    //             .then(self.0.cmp(&other.0))
    //     }
    // }
    //
    // impl PartialOrd for State {
    //     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    //         Some(self.cmp(other))
    //     }
    // }

    let (mut east_queue, mut south_queue) = grid
        .iter()
        .filter_map(|(p, c)| c.0.as_ref().map(|d| (*d, p)))
        .filter(|&(d, p)| {
            let p = d.step(p, &grid);
            grid.get(p.0, p.1).unwrap().0.is_none()
        })
        .partition_map::<Vec<_>, Vec<_>, _, _, _>(|(d, p)| match d {
            Dir::East => Either::Left(p),
            Dir::South => Either::Right(p),
        });

    east_queue.retain(|&p| {
        let n = Dir::East.step(p, &grid);
        grid.get(n.0, n.1).unwrap().0.is_none()
    });
    south_queue.retain(|&p| {
        let n = Dir::South.step(p, &grid);
        grid.get(n.0, n.1).unwrap().0.is_none()
    });

    let mut to_move = Movers {
        east: east_queue.into_iter().collect(),
        south: south_queue.into_iter().collect(),
    };

    let mut candidates = Movers::default();

    let mut counter = 1;
    while !to_move.east.is_empty() || !to_move.south.is_empty() {
        step(&mut to_move, &mut candidates, &mut grid);
        counter += 1;
    }

    counter
}

// fn print(grid: &Grid<Cell>, movers: &Movers) {
//     for (y, row) in grid.grid.iter().enumerate() {
//         for (x, c) in row.iter().enumerate() {
//             match c.0 {
//                 None => print!("."),
//                 Some(Dir::East) => {
//                     if movers.east.contains(&(x, y)) {
//                         print!("{}", ">".red());
//                     } else {
//                         print!(">");
//                     }
//                 },
//                 Some(Dir::South) => {
//                     if movers.south.contains(&(x, y)) {
//                         print!("{}", "v".blue());
//                     } else {
//                         print!("v");
//                     }
//                 }
//             }
//         }
//         println!();
//     }
// }

#[derive(Default, Debug, Clone)]
struct Movers {
    pub east: HashSet<(usize, usize)>,
    pub south: HashSet<(usize, usize)>,
}

fn step(to_move: &mut Movers, candidates: &mut Movers, grid: &mut Grid<Cell>) {
    for p in to_move.east.drain() {
        let n = Dir::East.step(p, grid);
        let b = Dir::East.unstep(p, grid);
        if grid.get(b.0, b.1).unwrap().0.is_some_and(|d| d.is_east()) {
            candidates.east.insert(b);
        }
        let be = Dir::South.unstep(p, grid);
        if grid
            .get(be.0, be.1)
            .unwrap()
            .0
            .is_some_and(|d| d.is_south())
        {
            to_move.south.insert(be);
        }

        grid.get_mut(p.0, p.1).unwrap().0 = None;
        grid.get_mut(n.0, n.1).unwrap().0 = Some(Dir::East);
        candidates.east.insert(n);
    }

    for p in to_move.south.drain() {
        let n = Dir::South.step(p, &grid);
        let b = Dir::South.unstep(p, &grid);
        if grid.get(b.0, b.1).unwrap().0.is_some_and(|x| x.is_south()) {
            candidates.south.insert(b);
        }
        let be = Dir::East.unstep(p, grid);
        if grid.get(be.0, be.1).unwrap().0.is_some_and(|d| d.is_east()) {
            candidates.east.insert(be);
        }

        // let g = grid.get_mut(n.0, n.1).unwrap();
        if grid.get(n.0, n.1).unwrap().0.is_none() {
            grid.get_mut(p.0, p.1).unwrap().0 = None;
            grid.get_mut(n.0, n.1).unwrap().0 = Some(Dir::South);
            candidates.south.insert(n);
        }
    }

    for p in candidates.east.drain() {
        let n = Dir::East.step(p, grid);
        if grid.get(n.0, n.1).unwrap().0.is_none() {
            to_move.east.insert(p);
        }
    }
    for p in candidates.south.drain() {
        let n = Dir::South.step(p, grid);
        if grid.get(n.0, n.1).unwrap().0.is_none() {
            to_move.south.insert(p);
        }
    }
}
