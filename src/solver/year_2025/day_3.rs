use std::io::BufRead;

use itertools::Itertools;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<u32> {
    let mut total = 0;
    for line in input.lines().map(Result::<String, _>::unwrap) {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let max_left = *digits[0..digits.len()-1].iter().max().unwrap();
        let i = digits.iter().find_position(|&&x| x == max_left).unwrap().0;
        let max_right = *digits[i+1..].iter().max().unwrap();
        total += max_left * 10 + max_right;
        continue;
    }

    Ok(total)
}

pub fn part_2(input: Box<dyn BufRead>) -> anyhow::Result<u64> {
    let mut total = 0;
    for line in input.lines().map(Result::<String, _>::unwrap) {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut joltage = 0;
        let n = 12;
        let mut cur_range = digits.as_slice();
        for i in 0..n {
            let search_range = cur_range.len() - (n - i) + 1;
            let max = cur_range[0..search_range].iter().copied().max().unwrap();
            let j = cur_range.iter().copied().find_position(|x| *x == max).unwrap().0;
            joltage = joltage * 10 + max as u64;
            cur_range = &cur_range[j+1..];
        }
        
        total += joltage;
    }

    Ok(total)
}