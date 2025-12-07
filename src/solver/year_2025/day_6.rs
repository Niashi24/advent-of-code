use anyhow::Context;
use std::{io::BufRead, str::FromStr};

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<u64> {
    let (operations, numbers) = parse(input)?;
    let part_1 = (0..numbers[0].len())
        .map(|x| {
            (0..numbers.len())
                .map(move |y| (x, y))
                .map(|(x, y)| numbers[y][x])
        })
        .zip(operations.iter())
        .map(|(col, op)| op.fold(col))
        .sum::<u64>();

    Ok(part_1)
}

pub fn part_2(input: Box<dyn BufRead>) -> anyhow::Result<u64> {
    let mut lines: Vec<String> = input.lines().collect::<Result<Vec<_>, _>>()?;
    // remove ending whitespace
    while lines
        .last()
        .map(|x| x.chars().all(|c| c.is_whitespace()))
        .unwrap_or(false)
    {
        lines.pop();
    }

    let mut operations = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<Operation>().unwrap());
    let mut cur_op = operations.next().unwrap();
    let mut cur_acc = cur_op.init();
    let mut total = 0;
    for x in 0..lines[0].len() {
        let num = lines[0..lines.len() - 1]
            .iter()
            .filter_map(|y| y[x..x + 1].parse::<u64>().ok())
            .fold(0, |acc, e| acc * 10 + e);

        if num == 0 {
            total += cur_acc;
            cur_op = operations.next().unwrap();
            cur_acc = cur_op.init();
            continue;
        }

        cur_acc = cur_op.apply(cur_acc, num);
    }

    total += cur_acc;

    Ok(total)
}

enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            c => Err(anyhow::anyhow!("unknown op: {c:?}")),
        }
    }
}

impl Operation {
    fn init(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Multiply => 1,
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }

    fn fold(&self, iter: impl Iterator<Item = u64>) -> u64 {
        match self {
            Self::Add => iter.fold(0, |acc, e| acc + e),
            Self::Multiply => iter.fold(1, |acc, e| acc * e),
        }
    }
}

fn parse(input: Box<dyn BufRead>) -> anyhow::Result<(Vec<Operation>, Vec<Vec<u64>>)> {
    let mut lines: Vec<String> = input.lines().collect::<Result<Vec<_>, _>>()?;

    let operations = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;

    let numbers = lines
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u64>().with_context(|| "parse: {s}"))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((operations, numbers))
}
