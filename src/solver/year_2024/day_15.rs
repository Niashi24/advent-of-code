use glam::IVec2;
use itertools::Itertools;
use std::fmt::Display;
use std::io::BufRead;
use utils::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Box,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn y_down(self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::East => IVec2::X,
            Direction::South => IVec2::Y,
            Direction::West => IVec2::NEG_X,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile2 {
    Wall,
    LeftBox,
    RightBox,
}

impl Tile2 {
    fn can_move(&self, p: IVec2, grid: &Grid<Option<Tile2>>, dir: Direction) -> bool {
        match (self, dir) {
            (Tile2::Wall, _) => false,
            (Tile2::RightBox, Direction::West) => match grid.get_i32(p.x - 2, p.y) {
                None => false,
                Some(None) => true,
                Some(Some(x)) => x.can_move(IVec2::new(p.x - 2, p.y), grid, dir),
            },
            (Tile2::LeftBox, Direction::East) => match grid.get_i32(p.x + 2, p.y) {
                None => false,
                Some(None) => true,
                Some(Some(x)) => x.can_move(IVec2::new(p.x + 2, p.y), grid, dir),
            },
            (Tile2::LeftBox, Direction::North) => {
                let a = match grid.get_i32(p.x, p.y - 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x, p.y - 1), grid, dir),
                };
                let b = match grid.get_i32(p.x + 1, p.y - 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x + 1, p.y - 1), grid, dir),
                };
                a && b
            }
            (Tile2::RightBox, Direction::North) => {
                let a = match grid.get_i32(p.x, p.y - 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x, p.y - 1), grid, dir),
                };
                let b = match grid.get_i32(p.x - 1, p.y - 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x - 1, p.y - 1), grid, dir),
                };
                a && b
            }
            (Tile2::LeftBox, Direction::South) => {
                let a = match grid.get_i32(p.x, p.y + 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x, p.y + 1), grid, dir),
                };
                let b = match grid.get_i32(p.x + 1, p.y + 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x + 1, p.y + 1), grid, dir),
                };
                a && b
            }
            (Tile2::RightBox, Direction::South) => {
                let a = match grid.get_i32(p.x, p.y + 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x, p.y + 1), grid, dir),
                };
                let b = match grid.get_i32(p.x - 1, p.y + 1) {
                    None => false,
                    Some(None) => true,
                    Some(Some(x)) => x.can_move(IVec2::new(p.x - 1, p.y + 1), grid, dir),
                };
                a && b
            }
            _ => panic!("{:?} {:?}", self, dir),
        }
    }

    fn mov(&self, p: IVec2, grid: &mut Grid<Option<Tile2>>, dir: Direction) {
        match (self, dir) {
            (Tile2::Wall, _) => {}
            (Tile2::RightBox, Direction::West) => {
                if let Some(t) = grid.get_i32(p.x - 2, p.y).unwrap().clone() {
                    t.mov(IVec2::new(p.x - 2, p.y), grid, dir);
                }
                *grid.get_i32_mut(p.x - 2, p.y).unwrap() = Some(Tile2::LeftBox);
                *grid.get_i32_mut(p.x - 1, p.y).unwrap() = Some(Tile2::RightBox);
                *grid.get_i32_mut(p.x, p.y).unwrap() = None;
            }
            (Tile2::LeftBox, Direction::East) => {
                if let Some(t) = grid.get_i32(p.x + 2, p.y).unwrap().clone() {
                    t.mov(IVec2::new(p.x + 2, p.y), grid, dir);
                }
                *grid.get_i32_mut(p.x + 2, p.y).unwrap() = Some(Tile2::RightBox);
                *grid.get_i32_mut(p.x + 1, p.y).unwrap() = Some(Tile2::LeftBox);
                *grid.get_i32_mut(p.x, p.y).unwrap() = None;
            }
            (Tile2::LeftBox, Direction::North) => {
                if let Some(t) = grid.get_i32(p.x, p.y - 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x, p.y - 1), grid, dir);
                }
                if let Some(t) = grid.get_i32(p.x + 1, p.y - 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x + 1, p.y - 1), grid, dir);
                }
                *grid.get_i32_mut(p.x, p.y - 1).unwrap() = Some(Tile2::LeftBox);
                *grid.get_i32_mut(p.x + 1, p.y - 1).unwrap() = Some(Tile2::RightBox);
                *grid.get_i32_mut(p.x, p.y).unwrap() = None;
                *grid.get_i32_mut(p.x + 1, p.y).unwrap() = None;
            }
            (Tile2::RightBox, Direction::North) => {
                if let Some(t) = grid.get_i32(p.x, p.y - 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x, p.y - 1), grid, dir);
                }
                if let Some(t) = grid.get_i32(p.x - 1, p.y - 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x - 1, p.y - 1), grid, dir);
                }
                *grid.get_i32_mut(p.x, p.y - 1).unwrap() = Some(Tile2::RightBox);
                *grid.get_i32_mut(p.x - 1, p.y - 1).unwrap() = Some(Tile2::LeftBox);
                *grid.get_i32_mut(p.x, p.y).unwrap() = None;
                *grid.get_i32_mut(p.x - 1, p.y).unwrap() = None;
            }
            (Tile2::LeftBox, Direction::South) => {
                if let Some(t) = grid.get_i32(p.x, p.y + 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x, p.y + 1), grid, dir);
                }
                if let Some(t) = grid.get_i32(p.x + 1, p.y + 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x + 1, p.y + 1), grid, dir);
                }
                *grid.get_i32_mut(p.x, p.y + 1).unwrap() = Some(Tile2::LeftBox);
                *grid.get_i32_mut(p.x + 1, p.y + 1).unwrap() = Some(Tile2::RightBox);
                *grid.get_i32_mut(p.x, p.y).unwrap() = None;
                *grid.get_i32_mut(p.x + 1, p.y).unwrap() = None;
            }
            (Tile2::RightBox, Direction::South) => {
                if let Some(t) = grid.get_i32(p.x, p.y + 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x, p.y + 1), grid, dir);
                }
                if let Some(t) = grid.get_i32(p.x - 1, p.y + 1).unwrap().clone() {
                    t.mov(IVec2::new(p.x - 1, p.y + 1), grid, dir);
                }
                *grid.get_i32_mut(p.x, p.y + 1).unwrap() = Some(Tile2::RightBox);
                *grid.get_i32_mut(p.x - 1, p.y + 1).unwrap() = Some(Tile2::LeftBox);
                *grid.get_i32_mut(p.x, p.y).unwrap() = None;
                *grid.get_i32_mut(p.x - 1, p.y).unwrap() = None;
            }
            _ => panic!("{:?} {:?}", self, dir),
        }
    }
}

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let mut robot = IVec2::new(0, 0);

    let mut lines = input.lines().map(Result::unwrap).peekable();

    let grid = lines.peeking_take_while(|x| !x.is_empty());

    let grid = grid
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => None,
                    'O' => Some(Tile::Box),
                    '#' => Some(Tile::Wall),
                    '@' => {
                        robot = IVec2::new(x as i32, y as i32);
                        None
                    }
                    _ => panic!("{c}"),
                })
                .collect_vec()
        })
        .collect::<Grid<Option<Tile>>>();
    let robot = robot;

    lines.next().unwrap();

    let moves = lines
        .flat_map(|l| {
            l.chars()
                .map(|c| match c {
                    '>' => Direction::East,
                    '<' => Direction::West,
                    'v' => Direction::South,
                    '^' => Direction::North,
                    _ => panic!("{c}"),
                })
                .collect_vec()
        })
        .collect_vec();

    let p_1 = {
        let mut robot = robot;
        let mut grid = grid.clone();
        for &m in &moves {
            let m = m.y_down();
            let mut new_pos = None;
            for i in 1.. {
                let new_p = m * i + robot;
                match grid.get_i32(new_p.x, new_p.y).copied() {
                    None | Some(Some(Tile::Wall)) => break,
                    Some(Some(Tile::Box)) => continue,
                    Some(None) => {
                        new_pos = Some(new_p);
                        break;
                    }
                }
            }
            let Some(new_pos) = new_pos else {
                continue;
            };

            *grid.get_i32_mut(new_pos.x, new_pos.y).unwrap() = Some(Tile::Box);

            *grid.get_i32_mut(robot.x + m.x, robot.y + m.y).unwrap() = None;
            robot += m;
        }

        grid.into_iter()
            .map(|((x, y), t)| match t {
                Some(Tile::Box) => 100 * y + x,
                _ => 0,
            })
            .sum::<usize>()
    };

    let p_2 = {
        let mut robot = IVec2::new(robot.x * 2, robot.y);
        let mut grid = grid
            .grid
            .clone()
            .into_iter()
            .map(|row| {
                row.into_iter().flat_map(|t| match t {
                    None => [None; 2],
                    Some(Tile::Wall) => [Some(Tile2::Wall); 2],
                    Some(Tile::Box) => [Some(Tile2::LeftBox), Some(Tile2::RightBox)],
                })
            })
            .collect::<Grid<_>>();

        for &m in &moves {
            let dir = m.y_down();
            let new_p = robot + dir;

            if match grid.get_i32(new_p.x, new_p.y).unwrap().clone() {
                None => true,
                Some(t) => {
                    if t.can_move(new_p, &grid, m) {
                        t.mov(new_p, &mut grid, m);
                        true
                    } else {
                        false
                    }
                }
            } {
                robot = new_p;
            }
        }

        grid.into_iter()
            .map(|((x, y), t)| match t {
                Some(Tile2::LeftBox) => 100 * y + x,
                _ => 0,
            })
            .sum::<usize>()
    };

    Ok((p_1, p_2))
}
