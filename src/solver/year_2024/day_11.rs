use hashbrown::HashMap;
use itertools::Itertools;
use std::fmt::Display;
use std::io::BufRead;
use utils::num_digits;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let s = input
        .lines()
        .flat_map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut memo = HashMap::new();

    let p_1 = s
        .iter()
        .copied()
        .map(|i| stones(i, 25, &mut memo))
        .sum::<usize>();
    let p_2 = s
        .iter()
        .copied()
        .map(|i| stones(i, 75, &mut memo))
        .sum::<usize>();

    Ok((p_1, p_2))
}

fn stones(i: u64, remaining: u8, memo: &mut HashMap<(u64, u8), usize>) -> usize {
    if remaining == 0 {
        return 1;
    }

    if let Some(&x) = memo.get(&(i, remaining)) {
        return x;
    }

    let out = if i == 0 {
        stones(1, remaining - 1, memo)
    } else {
        let digits = num_digits(i);
        if digits & 1 == 0 {
            stones(i / 10u64.pow(digits / 2), remaining - 1, memo)
                + stones(i % 10u64.pow(digits / 2), remaining - 1, memo)
        } else {
            stones(i * 2024, remaining - 1, memo)
        }
    };

    memo.insert((i, remaining), out);

    out
}
