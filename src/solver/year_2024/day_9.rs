use std::io::BufRead;
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day9;

impl CombinedSolver for Day9 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let line = input.lines().next().unwrap()?;
        let input = line
            .chars()
            .map(|c| c.to_digit(10).unwrap());
        
        let mut blocks = Vec::new();
        let mut free = false;
        let mut i = 0;
        for n in input.clone() {
            if !free {
                blocks.extend((0..n).map(|_| Some(i)));
                i += 1;
            } else {
                blocks.extend((0..n).map(|_| None));
            }
            
            free = !free;
        }
        
        let mut left = 0;
        
        while left < blocks.len() - 1 {
            if blocks[left].is_some() {
                left += 1;
                continue;
            }
            
            match blocks.pop().unwrap() {
                None => {
                    continue;
                }
                Some(x) => {
                    blocks[left] = Some(x);
                    left += 1;
                }
            }
        }
        
        let p_1 = blocks.iter().copied().enumerate()
            .map(|(i, n)| i as u64 * n.unwrap_or_default() as u64)
            .sum::<u64>();
        
        let mut blocks_2 = Vec::new();
        let mut free = false;
        let mut i = 0;
        for n in input {
            if !free {
                blocks_2.push((Some(i), n));
                i += 1;
            } else {
                blocks_2.push((None, n));
            }

            free = !free;
        }
        
        let mut right = blocks_2.len() - 1;
        while right > 0 {
            if blocks_2[right].0.is_none() {
                right -= 1;
                continue;
            };
            for i in 0..right {
                if blocks_2[i].0.is_none() && blocks_2[i].1 >= blocks_2[right].1 {
                    blocks_2[i].1 -= blocks_2[right].1;
                    if blocks_2[i].1 == 0 {
                        blocks_2[i] = blocks_2[right];
                    } else {
                        blocks_2.insert(i, blocks_2[right]);
                        right += 1;
                    }
                    
                    if blocks_2[right - 1].0.is_none() {
                        blocks_2[right - 1].1 += blocks_2.remove(right).1;
                    } else {
                        blocks_2[right].0 = None;
                    }
                    
                    break;
                }
            }
            right -= 1;
        }
        
        let mut i = 0;
        let mut p_2 = 0;
        for (value, length) in blocks_2 {
            if let Some(val) = value {
                for _ in 0..length {
                    p_2 += val as u64 * i as u64;
                    i += 1;
                }
            } else {
                i += length;
            }
        }
        
        Ok((p_1.to_string(), p_2.to_string()))
    }
}
