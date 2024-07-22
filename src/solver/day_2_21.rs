use std::io::BufRead;
use crate::day::SeparatedSolver;

pub struct Day221;

impl SeparatedSolver for Day221 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let (mut x, mut y): (i32, i32) = (0, 0);

        for (command, units) in input.lines()
            .map(|l| {
                let l = l.unwrap();
                let (command, units) = l.split_once(" ").unwrap();

                (command.to_string(), units.parse::<i32>().unwrap())
            }) {
            match command.as_str() {
                "forward" => x += units,
                "down" => y += units,
                "up" => y -= units,
                x => panic!("unknown command {}", x)
            }
        }

        Ok((x * y).to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let (mut x, mut y, mut aim): (i32, i32, i32) = (0, 0, 0);

        for (command, units) in input.lines()
            .map(|l| {
                let l = l.unwrap();
                let (command, units) = l.split_once(" ").unwrap();

                (command.to_string(), units.parse::<i32>().unwrap())
            }) {
            match command.as_str() {
                "forward" => { x += units; y += aim * units; },
                "down" => aim += units,
                "up" => aim -= units,
                x => panic!("unknown command {}", x)
            }
        }

        Ok((x * y).to_string())
    }
}