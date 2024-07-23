use std::fmt::{Display, Formatter};
use std::io::BufRead;

use enumset::{EnumSet, EnumSetType};
use itertools::Itertools;

use crate::day::SeparatedSolver;

pub struct Day821;

impl SeparatedSolver for Day821 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let part_1 = input.lines()
            .map(Result::unwrap)
            .map(|line| {
                let (_, output) = line.split_once(" | ").unwrap();
                output.split(" ")
                    .map(str::len)
                    .filter(|&n| n == 2 || n == 3 || n == 4 || n == 7)
                    .count()
            })
            .sum::<usize>();

        Ok(part_1.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let part_2 = input.lines()
            .map(Result::unwrap)
            .map(|line| {
                let (input, out) = line.split_once(" | ").unwrap();
                let input: Vec<EnumSet<Segments>> = input.split(" ")
                    .map(|s| s.chars().map(|c| Segments::try_from(c).unwrap()).collect())
                    .collect_vec();

                let one = *input.iter().find(|i| i.len() == 2).unwrap();
                let seven = *input.iter().find(|i| i.len() == 3).unwrap();
                let four = *input.iter().find(|i| i.len() == 4).unwrap();
                let eight = *input.iter().find(|i| i.len() == 7).unwrap();

                let six = *input.iter()
                    .filter(|i| i.len() == 6)
                    .filter(|i| i.intersection(one).len() == 1)
                    .next().unwrap();

                let c = eight.difference(six);
                let five = *input.iter()
                    .filter(|i| i.len() == 5)
                    .filter(|i| i.is_disjoint(c))
                    .next().unwrap();
                let e = six.difference(five);
                let nine = *input.iter()
                    .filter(|i| i.len() == 6)
                    .filter(|i| i.is_disjoint(e))
                    .next().unwrap();

                let (mut two, mut three) = input.iter()
                    .filter(|i| i.len() == 5)
                    .filter(|i| **i != five)
                    .copied()
                    .next_tuple::<(_, _)>().unwrap();

                if !two.is_superset(e) {
                    (two, three) = (three, two);
                }

                let values = [one, two, three, four, five, six, seven, eight, nine];

                out.split(" ")
                    .map(|s| s.chars().map(|c| Segments::try_from(c).unwrap()).collect::<EnumSet<_>>())
                    .map(|s| values.iter().position(|x| *x == s)
                        .map(|i| i + 1)
                        .unwrap_or(0))
                    .fold(0, |acc, i| acc * 10 + i)
            })
            .sum::<usize>();

        Ok(part_2.to_string())
    }
}

#[derive(Debug, EnumSetType)]
enum Segments {
    A, B, C, D, E, F, G
}

impl TryFrom<char> for Segments {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Segments as S;
        match value {
            'a' => Ok(S::A),
            'b' => Ok(S::B),
            'c' => Ok(S::C),
            'd' => Ok(S::D),
            'e' => Ok(S::E),
            'f' => Ok(S::F),
            'g' => Ok(S::G),
            _ => Err(value)
        }
    }
}

impl Display for Segments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Segments::A => 'a',
            Segments::B => 'b',
            Segments::C => 'c',
            Segments::D => 'd',
            Segments::E => 'e',
            Segments::F => 'f',
            Segments::G => 'g',
        })
    }
}
