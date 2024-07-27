use std::io::{BufRead, Read};
use std::ops::{Range, RangeInclusive};
use itertools::Itertools;
use crate::day::SeparatedSolver;

pub struct Day1721;

impl SeparatedSolver for Day1721 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let line = input.lines().next().unwrap().unwrap();
        let (_, y_r) = parse_ranges(&line);
        let part_1 = (*y_r.start() + 1).abs();
        let part_1 = part_1 * (part_1 + 1) / 2;
        Ok(part_1.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let line = input.lines().next().unwrap().unwrap();
        let (x_r, y_r) = parse_ranges(&line);
        let (x, y) = solution_range(x_r.clone(), y_r.clone());
        let part_2 = x.cartesian_product(y)
            .filter(|&(x, y)| hits(x, y, x_r.clone(), y_r.clone()))
            .count();
        
        Ok(part_2.to_string())
    }
}

fn solution_range(x_r: RangeI, y_r: RangeI) -> (RangeI, RangeI) {
    (0..=*x_r.end(), ((*y_r.start())..=(-y_r.start() + 1)))
}

fn parse_ranges(s: &str) -> (RangeI, RangeI) {
    // Extract the relevant part of the string
    let coords = s.strip_prefix("target area: ")
        .expect("String does not start with 'target area: '");

    // Split the coordinates into x and y parts
    let (x_part, y_part) = coords.split_once(", ").expect("Invalid format: missing comma separator");

    // Split and parse the x range
    let x_range = {
        let x_bounds = x_part.strip_prefix("x=").expect("Missing 'x=' prefix")
            .split_once("..").expect("Invalid x range format");
        let start = x_bounds.0.parse().expect("Invalid number in x range");
        let end = x_bounds.1.parse().expect("Invalid number in x range");
        start..=end
    };

    // Split and parse the y range
    let y_range = {
        let y_bounds = y_part.strip_prefix("y=").expect("Missing 'y=' prefix")
            .split_once("..").expect("Invalid y range format");
        let start = y_bounds.0.parse().expect("Invalid number in y range");
        let end = y_bounds.1.parse().expect("Invalid number in y range");
        start..=end
    };

    (x_range, y_range)
}

type RangeI = RangeInclusive<i32>;

fn hits(mut x_v: i32, mut y_v: i32, range_x: RangeI, range_y: RangeI) -> bool {
    let (mut x, mut y) = (0,0);
    
    while y >= *range_y.start() && x <= *range_x.end() {
        if range_x.contains(&x) && range_y.contains(&y) {
            return true;
        }
        x += x_v;
        y += y_v;
        x_v = (x_v - 1).max(0);
        y_v -= 1;
    }
    
    false
}