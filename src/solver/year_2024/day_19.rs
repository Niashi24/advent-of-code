use std::fmt::Display;
use std::io::BufRead;
use hashbrown::HashMap;
use itertools::Itertools;

pub fn solve(input: Box<dyn BufRead>) -> anyhow::Result<(impl Display, impl Display)> {
    let (towels, designs) = parse(input);
    
    let p_1 = designs.iter()
        .filter(|design| can_make(design, &towels))
        .count();
    
    let p_2 = designs.iter()
        .map(|design| {
            let mut memo = HashMap::new();
            num_make(0, design, &towels, &mut memo)
        })
        .sum::<usize>();
    
    Ok((p_1, p_2))
}

fn can_make(design: &[Towel], towels: &[Vec<Towel>]) -> bool {
    if design.len() == 0 {
        return true;
    }
    
    for towel in towels {
        if towel.len() > design.len() {
            continue;
        }
        
        if design.iter().zip(towel.iter()).all(|(x, y)| x == y) {
            if can_make(&design[towel.len()..], towels) {
                return true;
            }
        }
    }
    
    false
}

fn num_make(idx: usize, design: &[Towel], towels: &[Vec<Towel>], memo: &mut HashMap<usize, usize>) -> usize {
    if idx >= design.len() {
        return 1;
    }
    
    if let Some(out) = memo.get(&idx) {
        return *out;
    }

    let mut out = 0;
    for towel in towels {
        if towel.len() > design.len() - idx {
            continue;
        }

        if design[idx..].iter().zip(towel.iter()).all(|(x, y)| x == y) {
            out += num_make(idx + towel.len(), design, towels, memo);
        }
    }

    memo.insert(idx, out);
    
    out
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Towel {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for Towel {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Self::White),
            'u' => Ok(Self::Blue),
            'b' => Ok(Self::Black),
            'r' => Ok(Self::Red),
            'g' => Ok(Self::Green),
            _ => Err(value),
        }
    }
}

fn parse(input: Box<dyn BufRead>) -> (Vec<Vec<Towel>>, Vec<Vec<Towel>>) {
    let mut lines = input.lines().map(Result::unwrap);
    let towels = lines.next().unwrap();
    let towels = towels.split(", ")
        .map(|s| s.chars().map(Towel::try_from).map(Result::unwrap).collect_vec())
        .collect_vec();
    
    lines.next().unwrap();
    
    let designs = lines
        .map(|c| c.chars().map(Towel::try_from).map(Result::unwrap).collect_vec())
        .collect_vec();

    (towels, designs)
}