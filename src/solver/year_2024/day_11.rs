use std::fmt::Display;
use std::io::BufRead;
use itertools::Itertools;
use hashbrown::HashMap;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let s = input.lines().flat_map(|l| l.unwrap()
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect_vec())
        .collect_vec();
    
    let mut memo_1 = HashMap::new();
    let p_1 = s.iter().copied()
        .map(|i| stones(i, 0, 25, &mut memo_1))
        .sum::<usize>();

    let mut memo_2 = HashMap::new();
    let p_2 = s.iter().copied()
        .map(|i| stones(i, 0, 75, &mut memo_2))
        .sum::<usize>();
    
    Ok((p_1, p_2))
}

fn stones(i: u64, blink: u8, target: u8, memo: &mut HashMap<(u64, u8), usize>) -> usize {
    if blink == target {
        return 1;
    }
    
    if let Some(&x) = memo.get(&(i, blink)) {
        return x;
    }
    
    let out = if i == 0 {
        return stones(1, blink + 1, target, memo);
    } else {
        let digits = num_digits(i);
        if digits & 1 == 0 {
            stones(i / 10u64.pow(digits / 2), blink + 1, target, memo)
                + stones(i % 10u64.pow(digits / 2), blink + 1, target, memo)
        } else {
            stones(i * 2024, blink + 1, target, memo)
        }
    };
    
    memo.insert((i, blink), out);
    
    out
}

fn num_digits(n: u64) -> u32 {
    f32::log10(n as f32) as u32 + 1
}