use std::{env, io};
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use clap::{arg, command, Command, value_parser};
use rstest::rstest;
use thiserror::Error;
use crate::cli::{ExampleReader, ReadersError, RunArgs, RunType, SourceReader};
use crate::day::{Answer, Day, DayInfo, DaysMeta, DaySolver, Example, SolverDatabase};
use crate::RunError::NoSolver;

mod day;
mod cli;

fn main() {
    let args = RunType::parse();
    let meta = parse_meta(&Path::new("data/meta.json")).unwrap_or_default();

    dbg!(&args);
    dbg!(&meta);
    
    match args {
        RunType::Interactive => interactive(meta),
        RunType::Args(args) => { run_from_args(args, meta); },
    }
}

fn parse_meta(path: &Path) -> Option<DaysMeta> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);
    // should panic if unable to read meta
    Some(serde_json::from_reader(reader).unwrap())
}

fn interactive(meta: DaysMeta) {
    println!("Ran interactive");
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("no solver for {0}")]
    NoSolver(Day),
    #[error("no meta for {0}")]
    NoMeta(Day),
    #[error("wrong answer for example in part {part}: expected \"{expected}\" but was \"{actual}\"")]
    ExampleWrongAnswer {
        part: u8,
        expected: String,
        actual: String,
    },
}

// Situations needed to handle:
// Run only part 1 (example and full)
// Run only part 2 (example and full)
// Run both part 1 and 2
fn run_from_args(args: RunArgs, meta: DaysMeta) -> Result<RunResult, RunError> {
    let RunArgs { day, source, part } = args;
    
    let Some(solver) = SolverDatabase::default().get_solver(&day) else {
        return Err(NoSolver(day));
    };
    
    let reader = source.clone().to_readers(&meta, day)
        .map_err(|e| match e {
            ReadersError::NoMeta(day) => RunError::NoMeta(day),
            ReadersError::Io(io) => io.into(),
        })?;
    
    // match reader {
    //     SourceReader::Simple(reader) => {
    //         match solver {
    //             DaySolver::Combined(solver) => {
    //                 let now = Instant::now();
    //                 let result = solver.solve(reader);
    //                 let time = now.elapsed();
    //                 Ok(RunResult::Single(RunSingleResult {
    //                     result, time
    //                 }))
    //             }
    //             DaySolver::Separate(part_1, part_2) => {
    //                 let SourceReader::Simple(reader_2) = source.to_readers(&meta, day).unwrap() else {
    //                     unreachable!()
    //                 };
    //                 
    //                 let now = Instant::now();
    //                 let result_1 = part_1.solve(reader);
    //                 let time_1 = now.elapsed();
    //                 
    //                 let now = Instant::now();
    //                 let result_2 = part_2.solve(reader_2);
    //                 let time_2 = now.elapsed();
    //                 
    //                 Ok(RunResult::Multi(
    //                     RunSingleResult { result: result_1, time: time_1 },
    //                     RunSingleResult { result: result_2, time: time_2 },
    //                 ))
    //             }
    //         }
    //     }
    //     SourceReader::Example(example) => {
    //         match example {
    //             ExampleReader::Single { file, expected_answer } => {
    //                 let now = Instant::now();
    //                 let result = solver.solve(file);
    //                 let time = now.elapsed();
    //                 
    //                 if result.as_ref().is_ok_and(|a| a == &expected_answer ) {
    //                     return Err(RunError::ExampleWrongAnswer {
    //                         expected: expected_answer,
    //                         actual: result.unwrap(),
    //                     })
    //                 }
    //                 Ok(RunResult::Single(RunSingleResult {
    //                     result, time
    //                 }))
    //             }
    //             ExampleReader::Multi { file_1, expected_answer_1, file_2, expected_answer_2 } => {
    //                 
    //                 
    //                 
    //             }
    //         }
    //     }
    //     SourceReader::ExampleFull(_, _) => {}
    // }
    
    todo!()
}

pub enum RunResult {
    Single(RunSingleResult),
    Multi(RunSingleResult, RunSingleResult),
} 

pub struct RunSingleResult {
    pub result: day::Result,
    pub time: Duration,
}

// Requirements:
// Run in Interactive mode (no args)
//   View all:
//      When highlighted:
//          Run ▶ ▷
//          Visualization ★ ☆ 
//              Run from...
//                  Example (Optional)
//                  Full (Optional)
//                  File
//                  Text input
// Run by command line
// day (required): -d <day>
// year: -y <year>
// source:
//      Example -e
//      Full (default)
//      File -f <file>
//      Text -t <text>

// Examples of running the program
// ./app -> RunType::Interactive
// ./app -d 1 2022 -> RunType::Args(RunArgs { day: 1, year: 2022, source: RunSource::Full})
// ./app -d 3 2020 -e -> RunType::Args(RunArgs { day: 3, year: 2020, source: RunSource::Example })
// ./app -d 3 2020 -f "file.txt" -> RunType::Args(RunArgs { day: 3, year: 2020, source: RunSource::File(PathBuf("file.txt")) })
// ./app -d 3 2020 -t "123 123" -> RunType::Args(RunArgs { day: 3, year: 2020, source: RunSource::Text("123 123") })

