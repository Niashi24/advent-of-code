use std::io::BufRead;
use utils::extensions::FirstMax;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(u64, u64)> {
    let mut p_1 = 0;
    let mut p_2 = 0;
    for line in input.lines().map(Result::<String, _>::unwrap) {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        p_1 += solve_n(&digits, 2);
        p_2 += solve_n(&digits, 12);
    }

    Ok((p_1, p_2))
}

fn solve_n(digits: &[u32], n: usize) -> u64 {
    let mut joltage = 0;
    let mut cur_range = digits;
    for i in 0..n {
        let search_range = cur_range.len() - (n - i) + 1;
        let (j, max) = cur_range[0..search_range].iter().copied().first_max().unwrap();
        joltage = joltage * 10 + max as u64;
        cur_range = &cur_range[j+1..];
    }
    joltage
}
