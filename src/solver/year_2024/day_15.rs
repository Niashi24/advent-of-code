use std::fmt::Display;
use std::io::BufRead;
use glam::IVec2;
use itertools::Itertools;
use utils::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Fish,
}

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let mut robot = IVec2::new(0,0);
    
    let mut lines = input.lines().map(Result::unwrap).peekable();
    
    let mut grid = lines.peeking_take_while(|x| !x.is_empty());
    
    let grid = grid.enumerate()
        .map(|(y, l)| l.chars().enumerate().map(|(x, c)| match c {
            '.' => None,
            'O' => Some(Tile::Fish),
            '#' => Some(Tile::Wall),
            '@' => {
                robot = IVec2::new(x as i32, y as i32);
                None
            }
            _ => panic!("{c}"),
        }))
        .collect::<Grid<Option<Tile>>>();
    
    lines.next().unwrap();
    
    let moves = lines.next().unwrap().chars()
        .map(|c| match c {
            '>' => IVec2::X,
            '<' => IVec2::NEG_X,
            'v' => IVec2::Y,
            '^' => IVec2::NEG_Y,
            _ => panic!("{c}")
        })
        .collect_vec();

    let p_1 = {
        let mut grid = grid.clone();
        for &m in &moves {
            let Some(new_pos) = (1..)
                .map(|i| m * i + robot)
                .find(|p| grid.get_i32(p.x, p.y).is_none_or(|t| t.is_none()))
                else {
                    continue;
                };
            
            *grid.get_i32_mut(new_pos.x, new_pos.y) = Some(Tile::Fish);
            
            *grid.get_i32_mut(robot.x + m.x, robot.y + m.y) = None;
            robot += m;
        }
        
        grid.into_iter()
            .map(|((x, y), t)| match t {
                Some(Tile::Fish) => 100 * y + x,
                _ => 0,
            })
            .sum::<usize>()
    };
    
    Ok(p_1)
}