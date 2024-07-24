use std::cmp::PartialEq;
use std::io::BufRead;

use itertools::Itertools;

use crate::day::SeparatedSolver;

pub struct Day1021;

impl SeparatedSolver for Day1021 {

    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let total: usize = input.lines().map(Result::unwrap)
            .map(|s| parse_chunk(&s))
            .filter_map(|r| r.err())
            .map(Character::err_score)
            .sum();
        
        Ok(total.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let mut total = input.lines().map(Result::unwrap)
            .map(|s| parse_chunk(&s))
            .filter_map(Result::ok)
            .map(Character::score_all)
            .collect_vec();
        
        total.sort_unstable();
        
        Ok(total[total.len() / 2].to_string())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Character {
    Paren,
    Square,
    Curly,
    Angle,
}

impl Character {
    pub fn err_score(self) -> usize {
        match self {
            Character::Paren => 3,
            Character::Square => 57,
            Character::Curly => 1197,
            Character::Angle => 25137,
        }
    }
    
    pub fn inc_score(self) -> usize {
        match self {
            Character::Paren => 1,
            Character::Square => 2,
            Character::Curly => 3,
            Character::Angle => 4,
        }
    }
    
    pub fn score_all(stack: Vec<Self>) -> usize {
        stack.into_iter().rev()
            .map(Self::inc_score)
            .fold(0, |acc, c| acc * 5 + c)
    }
}

fn parse_chunk(s: &str) -> Result<Vec<Character>, Character> {
    let mut stack = vec![];
    
    for c in s.chars() {
        let (c, is_closing) = from_char(c);
        
        if is_closing {
            if stack.pop().unwrap() != c {
                return Err(c);
            }
        } else {
            stack.push(c);
        }
    }
    
    Ok(stack)
}

fn from_char(c: char) -> (Character, bool) {
    match c {
        '(' => (Character::Paren, false),
        ')' => (Character::Paren, true),
        '[' => (Character::Square, false),
        ']' => (Character::Square, true),
        '{' => (Character::Curly, false),
        '}' => (Character::Curly, true),
        '<' => (Character::Angle, false),
        '>' => (Character::Angle, true),
        _ => panic!("{c}")
    }
}

