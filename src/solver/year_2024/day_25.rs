use itertools::Itertools;
use std::fmt::Display;
use std::io::BufRead;

pub fn part_1(input: Box<dyn BufRead>) -> anyhow::Result<impl Display> {
    let (locks, keys) = parse(input);
    let mut out = 0;
    for lock in locks {
        for key in keys.clone() {
            if lock
                .into_iter()
                .zip(key.into_iter())
                .all(|(a, b)| a + b <= 7)
            {
                out += 1;
            }
        }
    }

    Ok(out)
}

fn parse(input: Box<dyn BufRead>) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut lines = input.lines().map(|x| x.unwrap());

    let mut lines = lines.peekable();

    let (mut locks, mut keys) = (Vec::new(), Vec::new());

    loop {
        let (x, lock) = parse_lock(lines.peeking_take_while(|x| !x.is_empty()));
        if lock {
            locks.push(x);
        } else {
            keys.push(x);
        }

        if lines.next().is_none() {
            break;
        }
    }

    (locks, keys)
}

fn parse_lock(mut lines: impl Iterator<Item = String>) -> ([u8; 5], bool) {
    let mut filled = [0; 5];
    let first = lines.next().unwrap();
    let lock = first.contains("#");
    if lock {
        filled = [1; 5];
    }

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                filled[i] += 1;
            }
        }
    }

    (filled, lock)
}
