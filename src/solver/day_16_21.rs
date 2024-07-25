use std::io::BufRead;
use crate::day::SeparatedSolver;

pub struct Day1621;

impl SeparatedSolver for Day1621 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {

        Ok("".to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        Ok("".to_string())
    }
}

struct BitParser<IT: Iterator<Item=bool>> {
    it: IT,
    i: usize,
}

impl<IT: Iterator<Item=bool>> From<IT> for BitParser<IT> {
    fn from(value: IT) -> Self {
        Self {
            it: value,
            i: 0,
        }
    }
}

impl<IT: Iterator<Item=bool>> BitParser<IT> {
    fn next_num(&mut self, n: u8) -> usize {
        let mut out = 0;

        for _ in 0..n {
            out = out << 1 + self.it.next().unwrap() as usize;
        }

        out
    }

    fn next_bit(&mut self) -> bool {
        self.it.next().unwrap()
    }
}
