use crate::day::CombinedSolver;
use colored::Colorize;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::io::BufRead;

pub struct Day4;

impl CombinedSolver for Day4 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let numbers = lines
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();

        lines.next();

        let mut boards = Vec::new();
        while let Some((a, b, c, d, e)) = lines.next_tuple() {
            boards.push(Board::from_arr([a, b, c, d, e].map(|s| {
                s.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })));

            lines.next();
        }

        let part_1 = part_1(numbers.as_slice(), boards.clone());
        let part_2 = part_2(numbers.as_slice(), boards.clone());

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

fn part_1(numbers: &[i32], mut boards: Vec<Board>) -> i32 {
    for &num in numbers {
        let mut completed = None;
        for board in boards.iter_mut() {
            if board.mark(num) {
                completed = Some(board.score());
                break;
            }
        }

        if let Some(score) = completed {
            return num * score;
        }
    }

    unreachable!()
}

fn part_2(numbers: &[i32], mut boards: Vec<Board>) -> i32 {
    for &num in numbers {
        if boards.len() > 1 {
            boards.retain_mut(|b| !b.mark(num));
        } else if boards.first_mut().unwrap().mark(num) {
            return num * boards.first().unwrap().score();
        }
    }
    unreachable!()
}

#[derive(Debug, Clone)]
pub struct Board {
    board: [[Tile; 5]; 5],
}

impl Board {
    pub fn from_arr(board: [[i32; 5]; 5]) -> Self {
        Self {
            board: board.map(|i| i.map(Tile::new)),
        }
    }

    pub fn from_vec(board: Vec<Vec<i32>>) -> Self {
        let board = board
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(Tile::new)
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Self { board }
    }

    pub fn try_get_pos(&self, num: i32) -> Option<(usize, usize)> {
        self.board
            .iter()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .flat_map(move |(x, t)| (t.0 == num).then_some((x, y)))
            })
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
        }
        // Assume that we haven't already gotten bingo
        false
    }

    pub fn score(&self) -> i32 {
        self.board
            .iter()
            .flat_map(|r| r.iter())
            .flat_map(|t| (!t.1).then_some(t.0))
            .sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for t in row.iter() {
                write!(f, "{} ", t)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tile(pub i32, pub bool);

impl Tile {
    pub fn new(n: i32) -> Self {
        Self(n, false)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:2}",
            if self.1 {
                self.0.to_string().red()
            } else {
                self.0.to_string().blue()
            }
        )
    }
}
