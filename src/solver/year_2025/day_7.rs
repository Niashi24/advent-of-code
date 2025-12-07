use std::io::BufRead;

use anyhow::{Context, bail};
use hashbrown::{HashMap, HashSet};

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(u32, u64)> {
    let (start, splitters) = parse(input)?;
    
    let mut active = HashMap::from([(start.0, 1u64)]);
    let mut buffer = HashMap::new();
    let max_y = splitters.iter().map(|&(_, y)| y).max().unwrap();
    let mut used_splitters = 0;
    for y in (start.1+1)..=max_y {
        for (x, n) in active.drain() {
            if splitters.contains(&(x, y)) {
                used_splitters += 1;
                *buffer.entry(x - 1).or_default() += n;
                *buffer.entry(x + 1).or_default() += n;
            } else {
                *buffer.entry(x).or_default() += n;
            }
        }

        core::mem::swap(&mut active, &mut buffer);
    }

    let timelines = active.values().sum::<u64>();
    
    Ok((used_splitters, timelines))
}

fn parse(input: Box<dyn BufRead>) -> anyhow::Result<((u32, u32), HashSet<(u32, u32)>)> {
    let mut start = None;
    let mut splitters = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let line: String = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {},
                'S' => start = Some((x as u32, y as u32)),
                '^' => { splitters.insert((x as u32, y as u32)); },
                _ => bail!("unknown {c:?} at ({x},{y})"),
            }
        }
    }

    Ok((start.with_context(|| "no start listed")?, splitters))
}