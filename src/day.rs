use crate::solver::year_2021::add_all;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::path::PathBuf;
use std::str::FromStr;

pub trait CombinedSolver: 'static + Sync {
    fn solve(&self, input: Box<dyn BufRead>) -> anyhow::Result<(String, String)>;
}

pub trait SeparatedSolver: 'static + Sync {
    fn part_1(&self, input: Box<dyn BufRead>) -> anyhow::Result<String>;
    fn part_2(&self, input: Box<dyn BufRead>) -> anyhow::Result<String>;
}

pub enum Solver {
    Combined(Box<dyn CombinedSolver>),
    Separated(Box<dyn SeparatedSolver>),
}

pub type Result = anyhow::Result<Answer>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Answer {
    P1(String),
    P2(String),
    Both(String, String),
}

impl Answer {
    pub fn passed(&self, other: &Answer) -> bool {
        match (self, other) {
            (Answer::Both(s_1, s_2), Answer::Both(p_1, p_2)) => s_1 == p_1 && s_2 == p_2,
            (Answer::P1(p1), Answer::P1(p2)) => p1 == p2,
            (Answer::P1(p1), Answer::Both(p2, _)) => p1 == p2,
            (Answer::P2(p1), Answer::Both(_, p2)) => p1 == p2,
            (Answer::P1(_), Answer::P2(_)) => true,
            _ => other.passed(self),
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::P1(p1) => write!(f, "P1: {}", p1),
            Answer::P2(p2) => write!(f, "P2: {}", p2),
            Answer::Both(p1, p2) => write!(f, "P1: {}, P2: {}", p1, p2),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Example {
    Single {
        path: PathBuf,
        expected_answer: Answer,
    },
    Multi {
        path_1: PathBuf,
        expected_answer_1: String,
        path_2: PathBuf,
        expected_answer_2: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DayInfo {
    pub full: PathBuf,
    pub example: Example,
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Day {
    pub day: i32,
    pub year: i32,
}

impl Day {
    #[inline]
    pub fn new(day: i32, year: i32) -> Self {
        Self { day, year }
    }
}

impl PartialOrd<Self> for Day {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year.cmp(&other.year).then(self.day.cmp(&other.day))
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.day, self.year)
    }
}

impl FromStr for Day {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (day, year) = s.split_once("-").ok_or(())?;
        Ok(Self {
            day: day.parse().map_err(|_| ())?,
            year: year.parse().map_err(|_| ())?,
        })
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DaysMeta(pub HashMap<String, DayInfo>);

impl DaysMeta {
    pub fn get_day(&self, day: Day) -> Option<&DayInfo> {
        self.0.get(&day.to_string())
    }
}

pub struct SolverDatabase {
    map: HashMap<Day, Solver>,
}

lazy_static! {
    static ref DATABASE: SolverDatabase = {
        let mut out = SolverDatabase {
            map: HashMap::new(),
        };
        add_all(&mut out);

        out
    };
}

impl SolverDatabase {
    pub fn global() -> &'static Self {
        &DATABASE
    }

    pub fn add_solver(&mut self, day: Day, solver: Solver) {
        self.map.insert(day, solver);
    }

    pub fn get_solver(&self, day: &Day) -> Option<&Solver> {
        self.map.get(day)
    }
}

// pub struct
