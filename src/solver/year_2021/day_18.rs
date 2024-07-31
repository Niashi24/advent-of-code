use std::fmt::{Display, Formatter};
use std::io::BufRead;

use itertools::Itertools;
use jiter::JsonValue;

use crate::day::SeparatedSolver;

pub struct Day18;

impl SeparatedSolver for Day18 {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let r = input
            .lines()
            .map(Result::unwrap)
            .map(|s| Number::from_str(&s))
            .reduce(Number::add)
            .unwrap();

        let part_1 = r.magnitude();
        Ok(part_1.to_string())
    }

    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String> {
        let numbers = input
            .lines()
            .map(Result::unwrap)
            .map(|s| Number::from_str(&s))
            .collect_vec();

        let part_2 = numbers
            .iter()
            .cartesian_product(numbers.iter())
            .map(|(a, b)| a.clone().add(b.clone()))
            .map(|a| a.magnitude())
            .max()
            .unwrap();

        Ok(part_2.to_string())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Number {
    Num(u32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn explode(&mut self, depth: u32, left: Option<&mut u32>, right: Option<&mut u32>) -> bool {
        if depth >= 4 {
            let Number::Pair(a, b) = self else {
                return false;
            };
            if let Some(l) = left {
                let Number::Num(a) = a.as_ref() else {
                    panic!("a was not num");
                };
                *l += *a;
            }
            if let Some(r) = right {
                let Number::Num(b) = b.as_ref() else {
                    panic!("b was not num");
                };
                *r += *b;
            }

            *self = Number::Num(0);

            true
        } else {
            match self {
                Number::Num(_) => false,
                Number::Pair(a, b) => {
                    let (a, b) = (a.as_mut(), b.as_mut());

                    a.explode(depth + 1, left, Some(b.leftest_mut()))
                        || b.explode(depth + 1, Some(a.rightest_mut()), right)
                }
            }
        }
    }

    fn leftest_mut(&mut self) -> &mut u32 {
        match self {
            Number::Num(i) => i,
            Number::Pair(a, _) => a.leftest_mut(),
        }
    }

    fn rightest_mut(&mut self) -> &mut u32 {
        match self {
            Number::Num(i) => i,
            Number::Pair(_, b) => b.rightest_mut(),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Num(i) => {
                if *i >= 10 {
                    let left = *i / 2;
                    let right = (*i + 1) / 2;

                    *self = Self::Pair(Self::Num(left).into(), Self::Num(right).into());

                    true
                } else {
                    false
                }
            }
            Number::Pair(a, b) => a.split() || b.split(),
        }
    }

    fn reduce_one(&mut self) -> bool {
        self.explode(0, None, None) || self.split()
    }

    fn reduce(&mut self) {
        while self.reduce_one() {}
    }

    fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let json = JsonValue::parse(bytes, false).unwrap();
        Self::from_json_value(json)
    }

    fn from_json_value(json_value: JsonValue) -> Self {
        match json_value {
            JsonValue::Array(x) => {
                let b = Self::from_json_value(x.last().unwrap().clone());
                let a = Self::from_json_value(x.first().unwrap().clone());

                Number::Pair(a.into(), b.into())
            }
            JsonValue::Int(x) => Number::Num(x as u32),
            x => panic!("{:?}", x),
        }
    }

    fn add(self, other: Self) -> Self {
        let mut out = Number::Pair(self.into(), other.into());

        out.reduce();

        out
    }

    fn magnitude(&self) -> usize {
        match self {
            Number::Num(i) => *i as usize,
            Number::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Num(i) => write!(f, "{}", i),
            Number::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}
