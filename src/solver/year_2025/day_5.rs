use std::io::BufRead;

use core::ops::RangeInclusive;

use itertools::Itertools;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<(usize, usize)> {
    let mut ranges: Vec<Ri> = Vec::new();
    let mut lines = input.lines().map(Result::unwrap);
    while let Some(line) = lines.next() {
        let line: String = line;
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once("-").unwrap();
        let mut range = start.parse::<i64>()?..=end.parse::<i64>()?;

        while let Some((i, _)) = ranges.iter().find_position(|r| intersects(r, &range)) {
            let r = ranges.swap_remove(i);
            range = merge(range, r);
        }

        ranges.push(range);
    }

    let p_1 = lines
        .map(|s: String| s.parse::<i64>().unwrap())
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count();

    let p_2 = ranges.iter().map(|r| r.end() - r.start() + 1).sum::<i64>() as usize;

    Ok((p_1, p_2))
}

type Ri = RangeInclusive<i64>;

fn intersects(a: &Ri, b: &Ri) -> bool {
    a.contains(b.start()) || b.contains(a.start())
}

fn merge(a: Ri, b: Ri) -> Ri {
    (*a.start()).min(*b.start())..=((*a.end()).max(*b.end()))
}
