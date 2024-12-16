use crate::day::CombinedSolver;
use memoize::memoize;
use std::io::BufRead;

pub struct Day21;

impl CombinedSolver for Day21 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let p_1 = lines.next().unwrap();
        let p_1: u32 = p_1
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap();
        let p_2 = lines.next().unwrap();
        let p_2: u32 = p_2
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap();

        let p_1: State = (p_1, 0);
        let p_2: State = (p_2, 0);

        let part_1 = run(p_1, p_2);

        let part_2 = recurse(p_1, p_2);
        let part_2 = part_2.0.max(part_2.1);

        Ok((part_1.to_string(), part_2.to_string()))
    }
}

fn run(mut p_1: State, mut p_2: State) -> usize {
    let mut dice = Dice::new();
    loop {
        p_1 = step(p_1, dice.next_total());
        if p_1.1 >= 1000 {
            return dice.n * (p_2.1 as usize);
        }
        p_2 = step(p_2, dice.next_total());
        if p_2.1 >= 1000 {
            return dice.n * (p_1.1 as usize);
        }
    }
}

#[memoize]
fn recurse(p_1: State, p_2: State) -> (usize, usize) {
    let mut w_1 = 0;
    let mut w_2 = 0;
    for (r_1, m_1) in ROLLS {
        for (r_2, m_2) in ROLLS {
            let p_1 = step(p_1, r_1);
            if p_1.1 >= 21 {
                w_1 += m_1;
                break;
            }
            let p_2 = step(p_2, r_2);
            if p_2.1 >= 21 {
                w_2 += m_2;
                continue;
            }
            let (r_1, r_2) = recurse(p_1, p_2);
            w_1 += m_1 * m_2 * r_1;
            w_2 += m_1 * m_2 * r_2;
        }
    }

    (w_1, w_2)
}

const ROLLS: [(u32, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

struct Dice {
    cur: u32,
    pub n: usize,
}

impl Dice {
    fn new() -> Self {
        Self { cur: 1, n: 0 }
    }

    fn next(&mut self) -> u32 {
        self.n += 1;
        let out = self.cur;
        self.cur += 1;
        if self.cur > 100 {
            self.cur = 1;
        }
        out
    }

    fn next_total(&mut self) -> u32 {
        self.next() + self.next() + self.next()
    }
}

type State = (u32, u32);

fn step((mut p, s): State, t: u32) -> State {
    p += t;
    while p > 10 {
        p -= 10;
    }

    (p, s + p)
}
