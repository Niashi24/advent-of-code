use std::io::BufRead;
use itertools::Itertools;
use crate::day::{CombinedSolver, SeparatedSolver};

pub struct Day321;

impl SeparatedSolver for Day321 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let lines = parse_lines(input);

        let len = lines.first().unwrap().len();

        let bits = (0..len)
            .rev()
            .map(|i| most_common_nth(&lines, i))
            .collect_vec();

        let gamma = from_bits(bits.iter().copied());
        let epsilon = from_bits(bits.iter().copied().map(|x| !x));

        Ok((gamma * epsilon).to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let lines = parse_lines(input);

        let oxygen = from_bits(process(lines.clone(), false).into_iter().rev());
        let co2 = from_bits(process(lines.clone(), true).into_iter().rev());

        Ok((oxygen * co2).to_string())
    }
}

fn parse_lines(input: Box<dyn BufRead>) -> Vec<Vec<bool>> {
    input.lines().map(Result::unwrap)
        .map(|l| l.chars().map(|c| c == '1').collect_vec())
        .collect_vec()
}

fn from_bits<It: IntoIterator<Item=bool>>(it: It) -> usize {
    it.into_iter().enumerate().map(|(i, b)| (1 << i) * b as usize).sum()
}

fn process(mut nums: Vec<Vec<bool>>, b: bool) -> Vec<bool> {
    let mut i = 0;
    while nums.len() > 1 {
        let x = most_common_nth(&nums, i) ^ b;
        nums.retain(|n| n[i] == x);
        i += 1;
    }
    nums.pop().unwrap()
}

// is 1 the most common bit in index i?
fn most_common_nth(nums: &Vec<Vec<bool>>, i: usize) -> bool {
    (nums.iter()
        .filter(|c| c[i])
        .count()) * 2 >= nums.len()
}

#[test]
fn should_be_most_common() {
    let given = vec![vec![true], vec![true], vec![false]];
    assert!(most_common_nth(&given, 0));
}

#[test]
fn should_be_least_common() {
    let given = vec![vec![false], vec![true], vec![false]];
    assert!(!most_common_nth(&given, 0));
}