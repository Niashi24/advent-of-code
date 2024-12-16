use std::collections::HashSet;
use std::io::BufRead;

use glam::IVec2;
use itertools::Itertools;

use crate::day::CombinedSolver;

pub struct Day20;

impl CombinedSolver for Day20 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let line = lines.next().unwrap();
        let algorithm = line.chars().map(|c| c == '#').collect_vec();
        lines.next(); // ignore empty line
        let map: Map = lines
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(x, _)| IVec2::new(x as i32, y as i32))
                    .collect_vec()
            })
            .collect();

        let part_1 = if !algorithm[0] {
            simple_solver(&algorithm, map.clone(), 2)
        } else {
            complex_solver(&algorithm, map.clone(), 1)
        };

        let part_2 = if !algorithm[0] {
            simple_solver(&algorithm, map, 50)
        } else {
            complex_solver(&algorithm, map, 25)
        };

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

type Map = HashSet<IVec2>;

fn simple_solver(algorithm: &[bool], mut map: Map, n: usize) -> usize {
    let mut buffer = Map::new();

    for _ in 0..n {
        (map, buffer) = simple_step(algorithm, map, buffer);
    }

    map.len()
}

fn simple_step(algorithm: &[bool], mut map: Map, mut buffer: Map) -> (Map, Map) {
    let (min_x, max_x) = map
        .iter()
        .copied()
        .map(|v| v.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = map
        .iter()
        .copied()
        .map(|v| v.y)
        .minmax()
        .into_option()
        .unwrap();

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let p = IVec2::new(x, y);
            let i = from_bits(NINE.into_iter().map(|n| n + p).map(|p| map.contains(&p)));

            if algorithm[i] {
                buffer.insert(p);
            }
        }
    }

    map.clear();

    (buffer, map)
}

fn complex_solver(algorithm: &[bool], mut map: Map, n: usize) -> usize {
    let mut buffer = Map::new();
    for _ in 0..n {
        (map, buffer) = simple_step(algorithm, map, buffer);
        (map, buffer) = flash_step(algorithm, map, buffer);
    }

    map.len()
}

fn flash_step(algorithm: &[bool], mut map: Map, mut buffer: Map) -> (Map, Map) {
    let (min_x, max_x) = map
        .iter()
        .copied()
        .map(|v| v.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = map
        .iter()
        .copied()
        .map(|v| v.y)
        .minmax()
        .into_option()
        .unwrap();

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            let p = IVec2::new(x, y);
            let i = from_bits(NINE.into_iter().map(|n| n + p).map(|p| {
                p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y || (map.contains(&(p)))
            }));

            if algorithm[i] {
                buffer.insert(p);
            }
        }
    }

    std::mem::swap(&mut map, &mut buffer);
    buffer.clear();

    (map, buffer)
}

fn from_bits(it: impl IntoIterator<Item = bool>) -> usize {
    let mut out = 0;
    for b in it {
        out <<= 1;
        out += b as usize;
    }
    out
}

const NINE: [IVec2; 9] = [
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
];
