use std::io::{BufRead, Read};
use regex::Regex;
use crate::day::CombinedSolver;

pub struct Day3;

impl CombinedSolver for Day3 {
    fn solve(&self, mut input: Box<dyn BufRead>) -> anyhow::Result<(String, String)> {
        let mut all = String::new();
        input.read_to_string(&mut all)?;
        
        let regex = Regex::new(r#"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"#)?;
        
        let mut p_1 = 0;
        let mut p_2 = 0;
        let mut enabled = true;

        for capture in regex.captures_iter(&all) {
            match &capture[0] {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    let a = capture[1].parse::<i32>()?;
                    let b = capture[2].parse::<i32>()?;
                    p_1 += a * b;
                    if enabled {
                        p_2 += a * b;
                    }
                }
            }
        }
        
        Ok((p_1.to_string(), p_2.to_string()))
    }
}