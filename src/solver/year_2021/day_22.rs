use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;
use crate::day::CombinedSolver;
use utils::ranges::RangeD;

pub struct Day22;

impl CombinedSolver for Day22 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let steps = input.lines().map(Result::unwrap)
            .map(|s| s.parse::<Step>().unwrap())
            .collect_vec();

        let part_1_volume = steps.iter()
            .filter(|s| s.1.start.iter().all(|s| *s >= -50) &&
            s.1.end.iter().all(|e| *e <= 51))
            .fold(State::new(), |acc, s| s.apply(acc));

        let part_1 = part_1_volume.into_iter()
            .map(|s| s.volume())
            .sum::<i64>();

        let part_2_volume = steps.iter()
            .fold(State::new(), |acc, s| s.apply(acc));

        let part_2 = part_2_volume.into_iter()
            .map(|s| s.volume())
            .sum::<i64>();


        Ok((part_1.to_string(), part_2.to_string()))
    }
}

#[derive(Debug, Clone)]
struct Step(bool, RangeD<3>);

type State = Vec<RangeD<3>>;

impl Step {
    fn apply(&self, state: State) -> State {
        let mut next_state = state.into_iter()
            .flat_map(|s| s.difference(&self.1)
                .into_iter())
            .collect_vec();

        if self.0 {
            next_state.push(self.1.clone());
        }

        next_state
    }
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((state, ranges)) = s.split_whitespace().next_tuple() else {
            return Err(());
        };

        let state = match state {
            "on" => true,
            "off" => false,
            _ => panic!("{state}")
        };

        let ranges = ranges.split(",")
            .map(|s| {
                let mut chars = s.chars();
                chars.next(); chars.next();
                let str = chars.as_str();
                let (min, max) = str.split_once("..").unwrap();
                let min = min.parse::<i64>().unwrap();
                let max = max.parse::<i64>().unwrap();
                min..(max + 1)
            })
            .collect_vec()
            .try_into()
            .unwrap();

        Ok(Self(
            state,
            RangeD::from_range_1d(ranges),
        ))
    }
}
