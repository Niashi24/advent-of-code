use std::io::BufRead;
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day421;

impl CombinedSolver for Day421 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {


        Ok(("".to_string(), "".to_string()))
    }
}

pub struct Board {
    board: [[Tile; 5]; 5]
}

impl Board {
    pub fn new(board: Vec<Vec<i32>>) -> Self {
        let board = board.into_iter()
            .map(|row| row.into_iter()
                .map(Tile::new)
                .collect_vec()
                .try_into().unwrap())
            .collect_vec()
            .try_into().unwrap();

        Self {
            board
        }
    }

    pub fn try_get_pos(&self, num: i32) -> Option<(usize, usize)> {
        self.board.iter().enumerate()
            .flat_map(move |(y, row)| row.iter()
                .enumerate()
                .flat_map(move |(x, t)| (t.0 == num).then_some((x, y))))
            .next()
    }

    pub fn mark(&mut self, num: i32) -> bool {
        if let Some((x, y)) = self.try_get_pos(num) {
            self.board[y][x].1 = true;

            // Check horizontals
            if (0..5).all(|x| self.board[y][x].1) {
                return true;
            }
            // Check verticals
            if (0..5).all(|y| self.board[y][x].1) {
                return true;
            }
            // Check horizontals
            if (0..5).all(|xy| self.board[xy][xy].1) {
                return true;
            }
            if (0..5).all(|xy| self.board[xy][4-xy].1) {
                return true;
            }

        }
        // Assume that we haven't already gotten bingo
        false
    }
}

#[derive(Debug)]
pub struct Tile(pub i32, pub bool);

impl Tile {
    pub fn new(n: i32) -> Self {
        Self(n, false)
    }
}