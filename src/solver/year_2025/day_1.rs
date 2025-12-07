use std::io::BufRead;

use anyhow::bail;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<i64> {
    let mut dial = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let val = {
            let line: String = line?;
            let mut chars = line.chars();
            let dir = match chars.next() {
                Some('L') => -1,
                Some('R') => 1,
                x => bail!("Unknown value: {x:?}"),
            };

            let v = chars.as_str().parse::<i64>()?;
            v * dir
        };

        dial = (dial + val).rem_euclid(100);

        if dial == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

pub fn part_2(input: Box<dyn BufRead>) -> anyhow::Result<i64> {
    let mut dial = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let val = {
            let line: String = line?;
            let mut chars = line.chars();
            let dir = match chars.next() {
                Some('L') => -1,
                Some('R') => 1,
                x => bail!("Unknown value: {x:?}"),
            };

            let v = chars.as_str().parse::<i64>()?;
            v * dir
        };

        let full_rots = val / 100;
        zeros += full_rots.abs();

        let old_pos = dial;
        dial = (dial + val).rem_euclid(100);

        // check if landed directly at zeros
        // or if we wrapped around (must have passed zero unless we were already at it)
        if dial == 0 || (old_pos != 0 && (dial > old_pos && val < 0) || (dial < old_pos && val > 0))
        {
            zeros += 1;
        }
    }

    Ok(zeros)
}
