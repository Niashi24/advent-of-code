use std::collections::HashMap;
use std::fmt::{Display, Formatter, write};
use std::io::BufRead;
use std::path::PathBuf;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

// Todo: uncombine?
pub trait Solver: 'static + Sync {
    fn solve(&self, input: Box<dyn BufRead>) -> Result;
}

pub type Result = anyhow::Result<Answer>;

pub enum DaySolver {
    Combined(Box<dyn Solver>),
    Separate(Box<dyn Solver>, Box<dyn Solver>)
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Answer {
    pub part_1: String,
    pub part_2: Option<String>,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.part_2 {
            None => write!(f, "({})", &self.part_1),
            Some(s) => write!(f, "({},{})", &self.part_1, s)
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

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.day, self.year)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DaysMeta(HashMap<Day, DayInfo>);

impl DaysMeta {
    pub fn get_day(&self, day: Day) -> Option<&DayInfo> {
        self.0.get(&day)
    }
}

pub struct SolverDatabase {
    map: HashMap<Day, DaySolver>,
}

lazy_static! {
    static ref DATABASE: SolverDatabase = {
        let mut map = HashMap::new();
        
        SolverDatabase {
            map
        }
    };
}

impl SolverDatabase {
    pub fn default() -> &'static Self {
        &DATABASE
    }

    pub fn get_solver(&self, day: &Day) -> Option<&DaySolver> {
        self.map.get(&day)
    }
}

// pub struct 
