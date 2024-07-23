use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::str::FromStr;
use glam::{IVec2, Vec2};
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day521;

impl CombinedSolver for Day521 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let lines = input.lines().map(Result::unwrap)
            .map(|l| l.parse::<Line>().unwrap())
            .collect_vec();
        
        let part_1 = solve(lines.iter()
            .filter_map(|l| l.straight_iter())
            .flat_map(|it| it));
        
        let part_2 = solve(lines.iter()
            .flat_map(|l| l.full_iter()));
        
        Ok((part_1.to_string(), part_2.to_string()))
    }
}

fn solve(it: impl Iterator<Item=IVec2>) -> usize {
    let mut points = HashSet::new();
    let mut finished = HashSet::new();
    for point in it {

        if !points.contains(&point) {
            points.insert(point);
        } else {
            finished.insert(point);
        }
    }
    
    finished.len()
}

struct Line(IVec2, IVec2);

impl Line {
    fn straight_iter(&self) -> Option<Box<dyn Iterator<Item=IVec2>>> {
        if self.0.x == self.1.x {
            let range = (self.0.y.min(self.1.y)..=self.0.y.max(self.1.y));
            let x = self.0.x;
            Some(Box::new(range.into_iter().map(move |y| IVec2::new(x, y)).into_iter()))
        } else if self.0.y == self.1.y {
            let range = (self.0.x.min(self.1.x)..=self.0.x.max(self.1.x));
            let y = self.0.y;
            Some(Box::new(range.into_iter().map(move |x| IVec2::new(x, y)).into_iter()))
        } else {
            None
        }
    }
    fn full_iter(&self) -> Box<dyn Iterator<Item=IVec2>> {
        if self.0.x == self.1.x {
            let range = (self.0.y.min(self.1.y)..=self.0.y.max(self.1.y));
            let x = self.0.x;
            Box::new(range.into_iter().map(move |y| IVec2::new(x, y)).into_iter())
        } else if self.0.y == self.1.y {
            let range = (self.0.x.min(self.1.x)..=self.0.x.max(self.1.x));
            let y = self.0.y;
            Box::new(range.into_iter().map(move |x| IVec2::new(x, y)).into_iter())
        } else {
            // Diagonal
            // let dif = (self.0.x - self.1.x).abs();
            let dir = (self.1 - self.0).signum();
            let n = (self.0.x - self.1.x).abs();
            let start = self.0;
            Box::new((0..=n).map(move |i| dir * i + start).into_iter())
            
            
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p_1, p_2) = s.split_once(" -> ").ok_or(())?;

        let (x_1, y_1) = p_1.split_once(",").ok_or(())?;
        let (x_2, y_2) = p_2.split_once(",").ok_or(())?;
        Ok(Line(
            IVec2::new(x_1.parse().map_err(|_| ())?, y_1.parse().map_err(|_| ())?),
            IVec2::new(x_2.parse().map_err(|_| ())?, y_2.parse().map_err(|_| ())?),
        ))
    }
}

