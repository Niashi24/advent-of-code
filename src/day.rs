use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::P1(p1) => write!(f, "({}, _)", p1),
            Answer::P2(p2) => write!(f, "(_, {})", p2),
            Answer::Both(p1, p2) => write!(f, "({}, {})", p1, p2),
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

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.day, self.year)
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
        let mut map = HashMap::new();
        map.insert(
            Day::new(1, 2022),
            Solver::Combined(Box::new(crate::solver::day_1_22::Day122)),
        );

        SolverDatabase { map }
    };
}

impl SolverDatabase {
    pub fn default() -> &'static Self {
        &DATABASE
    }

    pub fn get_solver(&self, day: &Day) -> Option<&Solver> {
        self.map.get(&day)
    }
}

// pub struct
