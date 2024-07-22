use crate::day::CombinedSolver;
use std::io::BufRead;

pub struct Day122;

impl CombinedSolver for Day122 {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        Ok(("solved".into(), "ok".into()))
    }
}
