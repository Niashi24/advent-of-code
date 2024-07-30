use std::collections::HashSet;
use std::io::BufRead;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use crate::day::CombinedSolver;

pub struct Day1321;

impl CombinedSolver for Day1321 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut lines = input.lines().map(Result::unwrap);
        let mut paper = lines.fold_while(Paper::new(), |mut acc, line| {
            if line.is_empty() {
                return Done(acc);
            }

            let (x, y) = line.split_once(",").unwrap();
            acc.insert((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));

            Continue(acc)
        }).into_inner();

        let folds = lines.map(|l| {
            let (_, l) = l.split_once("fold along ").unwrap();
            let (f, i) = l.split_once("=").unwrap();
            let f = match f {
                "x" => Dir::X,
                "y" => Dir::Y,
                _ => panic!("{f}")
            };

            Fold(f, i.parse().unwrap())
        })
            .collect_vec();

        folds.first().unwrap().fold(&mut paper);
        let part_1 = paper.len();

        for fold in folds.into_iter().skip(1) {
            fold.fold(&mut paper);
        }

        let part_2 = to_dot_paper(&paper);

        Ok((part_1.to_string(), part_2))
    }
}

fn to_dot_paper(paper: &Paper) -> String {
    let mut out = String::new();
    let (min_x, max_x) = paper.iter().copied().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = paper.iter().copied().map(|(_, y)| y).minmax().into_option().unwrap();
    for y in min_y..=max_y {
        out.push('\n');
        for x in min_x..=max_x {
            out.push(if paper.contains(&(x, y)) {
                '#'
            } else {
                '.'
            })
        }
    }

    out
}

type Paper = HashSet<(i32, i32)>;
#[derive(Copy, Clone, Debug)]
struct Fold(Dir, i32);

impl Fold {
    fn fold(&self, paper: &mut Paper) {
        let mut out = Paper::with_capacity(paper.len());
        match self.0 {
            Dir::X => {
                for (mut x, y) in paper.iter().copied() {
                    if x > self.1 {
                        x = 2 * self.1 - x;
                    }
                    out.insert((x, y));
                }
            }
            Dir::Y => {
                for (x, mut y) in paper.iter().copied() {
                    if y > self.1 {
                        y = 2 * self.1 - y;
                    }
                    out.insert((x, y));
                }
            }
        }

        *paper = out;
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir { X, Y }


