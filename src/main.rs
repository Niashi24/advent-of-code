use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

use itertools::Itertools;
use thiserror::Error;

use crate::cli::{ExampleReader, ReadersError, RunArgs, RunType, SourceReader};
use crate::day::{Answer, Day, DayInfo, DaysMeta, Solver, SolverDatabase};

pub mod cli;
pub mod day;
pub mod grid;
pub mod solver;

fn main() -> anyhow::Result<()> {
    let args = RunType::parse();
    let meta = parse_meta(Path::new("data/meta.json")).unwrap_or_default();

    match args {
        RunType::Interactive => interactive(meta),
        RunType::All => {
            run_all(&meta)?;
        }
        RunType::Args(args) => {
            let result = run_from_args(args, meta);
            match result {
                Ok(r) => println!("{}", r),
                Err(e) => println!("Err: {}", e),
            }
        }
    }

    Ok(())
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
#[error("No solver for {0}")]
pub struct NoSolver(Day);

fn run_all(meta: &DaysMeta) -> anyhow::Result<Duration> {
    let mut total = Duration::default();

    let mut days: Vec<(Day, &DayInfo)> = meta
        .0
        .iter()
        .map(|(s, i)| {
            let day = s.parse().unwrap();
            (day, i)
        })
        .collect_vec();

    days.sort_unstable_by_key(|x| x.0);

    for (day, info) in days {
        let full = File::open(&info.full)?;
        let full = Box::new(BufReader::new(full));

        let solver = SolverDatabase::global()
            .get_solver(&day)
            .ok_or(NoSolver(day))?;
        match solver {
            Solver::Combined(solver) => {
                let (r, t) = time_fn(|| solver.solve(full));
                let (p_1, p_2) = r?;
                println!("{day}: {t:.2?}");
                println!("    {}", p_1);
                println!("    {}", p_2);

                total += t;
            }
            Solver::Separated(solver) => {
                let (r_1, t_1) = time_fn(|| solver.part_1(full));
                let a_1 = r_1?;

                let full = File::open(&info.full)?;
                let full = Box::new(BufReader::new(full));
                let (r_2, t_2) = time_fn(|| solver.part_2(full));

                let a_2 = r_2?;

                println!("{day}: {:.2?}", t_1 + t_2);
                println!("    {} in {:.2?}", a_1, t_1);
                println!("    {} in {:.2?}", a_2, t_2);

                total += t_1;
                total += t_2;
            }
        }
    }

    println!("Finished all in {:.2?}", total);

    Ok(total)
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("no solver for {0}")]
    NoSolver(Day),
    #[error("no meta for {0}")]
    NoMeta(Day),
    #[error(
        "wrong answer for example in part {part}: expected \"{expected}\" but was \"{actual}\""
    )]
    ExampleWrongAnswer {
        part: Part,
        expected: Answer,
        actual: Answer,
    },
    #[error("Used a combined solver for a separated example")]
    CombinedForSeparatedAnswer,
}

#[derive(Debug)]
pub enum Part {
    P1,
    P2,
    Both,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Part::P1 => "1",
                Part::P2 => "2",
                Part::Both => "1&2",
            }
        )
    }
}

// Situations needed to handle:
// Run only part 1 (example and full)
// Run only part 2 (example and full)
// Run both part 1 and 2
fn run_from_args(args: RunArgs, meta: DaysMeta) -> Result<RunResult, RunError> {
    let RunArgs { day, source, part } = args;

    let Some(solver) = SolverDatabase::global().get_solver(&day) else {
        return Err(RunError::NoSolver(day));
    };

    let reader = source.clone().to_readers(&meta, day).map_err(|e| match e {
        ReadersError::NoMeta(day) => RunError::NoMeta(day),
        ReadersError::Io(io) => io.into(),
    })?;

    match reader {
        SourceReader::Simple(reader) => match solver {
            Solver::Combined(solver) => {
                let (result, time) = time_fn(|| {
                    solver
                        .solve(reader)
                        .map(|(s_1, s_2)| Answer::Both(s_1, s_2))
                });
                Ok(RunResult::Single(RunSingleResult { result, time }))
            }
            Solver::Separated(solver) => {
                let SourceReader::Simple(reader_2) = source.to_readers(&meta, day).unwrap() else {
                    unreachable!()
                };

                let (result_1, time_1) = time_fn(|| solver.part_1(reader));

                Ok(match result_1 {
                    Ok(part_1) => {
                        let (result_2, time_2) =
                            time_fn(|| solver.part_2(reader_2).map(Answer::P2));

                        RunResult::Multi(
                            RunSingleResult {
                                result: Ok(Answer::P1(part_1)),
                                time: time_1,
                            },
                            RunSingleResult {
                                result: result_2,
                                time: time_2,
                            },
                        )
                    }
                    Err(e) => RunResult::Single(RunSingleResult {
                        result: Err(e),
                        time: time_1,
                    }),
                })
            }
        },
        SourceReader::Example(example, full) => {
            match (solver, example) {
                (
                    Solver::Combined(solver),
                    ExampleReader::Single {
                        file,
                        expected_answer,
                    },
                ) => {
                    let (result, time) =
                        time_fn(|| solver.solve(file).map(|(s_1, s_2)| Answer::Both(s_1, s_2)));

                    if result.as_ref().is_ok_and(|a| !a.passed(&expected_answer)) {
                        return Err(RunError::ExampleWrongAnswer {
                            part: Part::Both,
                            expected: expected_answer,
                            actual: result.unwrap(),
                        });
                    }

                    if let Some(full) = full {
                        let (result, time) =
                            time_fn(|| solver.solve(full).map(|(a, b)| Answer::Both(a, b)));

                        Ok(RunResult::Single(RunSingleResult { result, time }))
                    } else {
                        Ok(RunResult::Single(RunSingleResult { result, time }))
                    }
                }
                (Solver::Combined(_), ExampleReader::Multi { .. }) => {
                    Err(RunError::CombinedForSeparatedAnswer)
                }
                (
                    Solver::Separated(solver),
                    ExampleReader::Multi {
                        file_1,
                        expected_answer_1,
                        file_2,
                        expected_answer_2,
                    },
                ) => {
                    let (r_1, t_1) = time_fn(|| solver.part_1(file_1));

                    if r_1.as_ref().is_ok_and(|a| a != &expected_answer_1) {
                        return Err(RunError::ExampleWrongAnswer {
                            part: Part::P1,
                            expected: Answer::P1(expected_answer_1),
                            actual: Answer::P1(r_1.unwrap()),
                        });
                    }
                    if r_1.is_err() {
                        return Ok(RunResult::Single(RunSingleResult {
                            result: r_1.map(Answer::P1),
                            time: t_1,
                        }));
                    }

                    let (r_2, t_2) = time_fn(|| solver.part_2(file_2));
                    if r_2.as_ref().is_ok_and(|a| a != &expected_answer_2) {
                        return Err(RunError::ExampleWrongAnswer {
                            part: Part::P2,
                            expected: Answer::P2(expected_answer_2),
                            actual: Answer::P2(r_2.unwrap()),
                        });
                    }

                    if let Some(full) = full {
                        let Ok(SourceReader::Example(_, Some(full_2))) =
                            source.to_readers(&meta, day)
                        else {
                            unreachable!();
                        };

                        let (r_1, t_1) = time_fn(|| solver.part_1(full).map(Answer::P1));
                        let (r_2, t_2) = time_fn(|| solver.part_2(full_2).map(Answer::P2));

                        Ok(RunResult::Multi(
                            RunSingleResult {
                                result: r_1,
                                time: t_1,
                            },
                            RunSingleResult {
                                result: r_2,
                                time: t_2,
                            },
                        ))
                    } else {
                        Ok(RunResult::Multi(
                            RunSingleResult {
                                result: r_1.map(Answer::P1),
                                time: t_1,
                            },
                            RunSingleResult {
                                result: r_2.map(Answer::P2),
                                time: t_2,
                            },
                        ))
                        // let r: day::Result = {
                        //     match (r_1, r_2) {
                        //         (Ok(a), Ok(b))
                        //     }
                        // }
                    }
                }
                (
                    Solver::Separated(solver),
                    ExampleReader::Single {
                        file,
                        expected_answer,
                    },
                ) => {
                    let Ok(SourceReader::Example(
                        ExampleReader::Single { file: file_2, .. },
                        full_2,
                    )) = source.to_readers(&meta, day)
                    else {
                        unreachable!();
                    };

                    let (r_1, t_1) = time_fn(|| solver.part_1(file));

                    let (r_2, t_2) = time_fn(|| solver.part_2(file_2));

                    // dbg!(&r_1, &r_2, &expected_answer);

                    match (r_1, r_2) {
                        (Ok(a_1), Ok(a_2)) => {
                            let answer = Answer::Both(a_1, a_2);
                            if !answer.passed(&expected_answer) {
                                return Err(RunError::ExampleWrongAnswer {
                                    part: Part::Both,
                                    expected: expected_answer,
                                    actual: answer,
                                });
                            } else if let Some(full) = full {
                                let (r_1, t_1) = time_fn(|| solver.part_1(full));

                                let (r_2, t_2) = time_fn(|| solver.part_2(full_2.unwrap()));

                                return Ok(RunResult::Multi(
                                    RunSingleResult {
                                        result: r_1.map(Answer::P1),
                                        time: t_1,
                                    },
                                    RunSingleResult {
                                        result: r_2.map(Answer::P2),
                                        time: t_2,
                                    },
                                ));
                            }
                            let Answer::Both(a_1, a_2) = answer else {
                                unreachable!()
                            };
                            let r_1 = Ok(a_1);
                            let r_2 = Ok(a_2);

                            Ok(RunResult::Multi(
                                RunSingleResult {
                                    result: r_1.map(Answer::P1),
                                    time: t_1,
                                },
                                RunSingleResult {
                                    result: r_2.map(Answer::P2),
                                    time: t_2,
                                },
                            ))
                        }
                        (r_1, r_2) => Ok(RunResult::Multi(
                            RunSingleResult {
                                result: r_1.map(Answer::P1),
                                time: t_1,
                            },
                            RunSingleResult {
                                result: r_2.map(Answer::P2),
                                time: t_2,
                            },
                        )),
                    }
                }
            }
        }
    }
}

#[inline]
fn time_fn<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let now = Instant::now();
    let result = f();
    let time = now.elapsed();
    (result, time)
}

pub enum RunResult {
    Single(RunSingleResult),
    Multi(RunSingleResult, RunSingleResult),
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RunResult::Single(r) => {
                write!(f, "{}", r)
            }
            RunResult::Multi(r_1, r_2) => {
                write!(f, "Result:\n  {}\n  {}", r_1, r_2)
            }
        }
    }
}

pub struct RunSingleResult {
    pub result: day::Result,
    pub time: Duration,
}

impl Display for RunSingleResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.result {
            Ok(a) => write!(f, "Obtained result: {} in {:.2?}", a, self.time),
            Err(e) => write!(f, "Failed: {}", e),
        }
    }
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
