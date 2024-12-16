use std::fmt::Display;
use std::io::BufRead;
use itertools::Itertools;
use nalgebra::matrix;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    Ok(input.lines()
        .map(Result::unwrap)
        .chunks(4).into_iter()
        .map(|mut c| {
            let b_1 = c.next().unwrap();
            let b_p = b_1.split_once(", Y+").unwrap();
            let x_1 = b_p.0.strip_prefix("Button A: X+").unwrap().parse::<i64>().unwrap();
            let y_1 = b_p.1.parse::<i64>().unwrap();
            
            let b_2 = c.next().unwrap();
            let b_p = b_2.split_once(", Y+").unwrap();
            let x_2 = b_p.0.strip_prefix("Button B: X+").unwrap().parse::<i64>().unwrap();
            let y_2 = b_p.1.parse::<i64>().unwrap();

            let prize = c.next().unwrap();
            let b_p = prize.split_once(", Y=").unwrap();
            let x_p = b_p.0.strip_prefix("Prize: X=").unwrap().parse::<i64>().unwrap();
            let y_p = b_p.1.parse::<i64>().unwrap();

            (x_1, y_1, x_2, y_2, x_p, y_p)
        })
        .fold((0, 0), |(p_1, p_2), (x_1, y_1, x_2, y_2, x_p, y_p)| {
            (
                p_1 + solve_eqn(x_1, y_1, x_2, y_2, x_p, y_p).unwrap_or_default(),
                p_2 + solve_eqn(x_1, y_1, x_2, y_2, x_p + 10000000000000, y_p + 10000000000000).unwrap_or_default(),
            )
        }))
}

fn solve_eqn(x_1: i64, y_1: i64, x_2: i64, y_2: i64, x_p: i64, y_p: i64) -> Option<i64> {
    // solve [X; Y] * [N] = [P] ==> [N] = [X; Y]^-1 * P
    // simple 2x2 matrix inverse
    let n = matrix![y_2, -x_2; -y_1, x_1] * matrix![x_p; y_p];
    let determinant = x_1 * y_2 - x_2 * y_1;
    if determinant == 0 || n[0] % determinant != 0 || n[1] % determinant != 0 {
        None
    } else {
        let n = n / determinant;
        Some(n[0] * 3 + n[1])
    }
}
