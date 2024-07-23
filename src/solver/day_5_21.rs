use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

use glam::IVec2;
use itertools::Itertools;

use crate::day::CombinedSolver;

pub struct Day521;

impl CombinedSolver for Day521 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let lines = input.lines().map(Result::unwrap)
            .map(|l| l.parse::<Line>().unwrap())
            .collect_vec();
        
        let part_1 = solve(lines.iter()
            .filter(|l| l.0.x == l.1.x || l.0.y == l.1.y)
            .flat_map(|l| l.iter()));
        
        let part_2 = solve(lines.iter()
            .flat_map(|l| l.iter()));
        
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
    fn iter(&self) -> impl Iterator<Item=IVec2> {
        let dir = (self.1 - self.0).signum();
        let n = (self.0.x - self.1.x).abs().max((self.0.y - self.1.y).abs());
        let start = self.0;
        (0..=n).map(move |i| dir * i + start).into_iter()
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

