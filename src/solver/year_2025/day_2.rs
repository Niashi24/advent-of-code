use std::io::BufRead;

use anyhow::{Context};

pub fn solve(mut input: Box<dyn BufRead>) -> anyhow::Result<(i64, i64)> {
    let mut line = String::new();
    input.read_to_string(&mut line)?;
    let line = line.trim();

    let mut total_1 = 0;
    let mut total_2 = 0;
    for range_str in line.split(",") {
        let (start, end) = {
            let (start, end) = range_str.split_once("-").with_context(|| "no range delimiter for {range_str}")?;

            (start.parse::<i64>()?, end.parse::<i64>()?)
        };

        for id in start..=end {
            if is_invalid_id(id) {
                total_1 += id;
            }
            if is_invalid_id_2(id) {
                total_2 += id;
            }
        }
        
    }

    Ok((total_1, total_2))
}

fn is_invalid_id(i: i64) -> bool {
    let s = i.to_string();
    if s.len() % 2 == 1 {
        return false;
    }

    let (s, e) = s.split_at(s.len() / 2);

    s == e
}

fn is_invalid_id_2(i: i64) -> bool {
    let s = i.to_string();

    'outer: for i in 2..=s.len() {
        if s.len() % i != 0 {
            continue;
        }

        let (start, mut next) = s.split_at(s.len() / i);
        for _ in 0..(i-1) {
            let here;
            (here, next) = next.split_at(s.len() / i);
            if start != here {
                continue 'outer;
            }
        }

        return true;
    }

    return false;
}

#[test]
fn test_is_invalid_id_2() {
    assert!(is_invalid_id_2(11));
    assert!(is_invalid_id_2(22));
    assert!(is_invalid_id_2(999));
    assert!(is_invalid_id_2(1010));
    assert!(!is_invalid_id_2(12));
    assert!(!is_invalid_id_2(1011));
    assert!(is_invalid_id_2(1111111));
}