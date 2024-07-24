use std::collections::HashSet;
use std::io::BufRead;
use itertools::Itertools;
use crate::day::SeparatedSolver;
use crate::grid::Grid;

pub struct Day1121;

impl SeparatedSolver for Day1121 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let grid: Grid<u8> = input.lines().map(Result::unwrap)
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect_vec())
            .collect();
        
        Ok("".to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        Ok("".to_string())
    }
}

fn step(mut grid: Grid<u8>) -> (Grid<u8>, usize) {
    let flashed = HashSet::<(usize, usize)>::new();
    
    todo!()
} 

