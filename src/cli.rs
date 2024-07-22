use crate::day::{Answer, Day, DaysMeta, Example};
use clap::{arg, command, value_parser, Command};
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::path::PathBuf;
use std::{env, io};
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RunType {
    Interactive,
    Args(RunArgs),
    All,
}

impl RunType {
    pub fn parse() -> Self {
        Self::parse_from(env::args_os())
    }

    pub fn command() -> Command {
        command!()
            .arg(
                arg!(
                    -d --day <DAY> "The day to run"
                )
                .required(false)
                .value_parser(value_parser!(i32)),
            )
            .arg(
                arg!(
                    -y --year <YEAR> "The year to run"
                )
                .required(false)
                .value_parser(value_parser!(i32)),
            )
            .arg(
                arg!(
                    -e --example "Run example only"
                )
                .required(false),
            )
            .arg(
                arg!(
                    -a --all "Run all days"
                )
                .required(false),
            )
            .arg(
                arg!(
                    -m --main "Run main input"
                )
                .required(false),
            )
            .arg(
                arg!(
                    -f --file <FILE> "Run with file as input"
                )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -t --text <TEXT> "Run with text as input"
                )
                .required(false)
                .value_parser(value_parser!(String)),
            )
            .arg(arg!(
                -'1' --one "Run part 1"
            ))
            .arg(arg!(
                -'2' --two "Run part 2"
            ))
    }

    pub fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = Self::command().get_matches_from(itr);

        if matches.get_flag("all") {
            RunType::All
        } else if let (Some(&day), Some(&year)) = (
            matches.get_one::<i32>("day"),
            matches.get_one::<i32>("year"),
        ) {
            let part = match (matches.get_flag("one"), matches.get_flag("two")) {
                (true, true) | (false, false) => PartArgs::Both,
                (true, false) => PartArgs::P1,
                (false, true) => PartArgs::P2,
            };

            let source = if matches.get_flag("main") {
                RunSource::Example(ExampleSource::Main)
            } else if matches.get_flag("example") {
                RunSource::Example(ExampleSource::ExampleOnly)
            } else if let Some(file) = matches.get_one::<PathBuf>("file") {
                RunSource::Single(SingleSource::File(file.clone()))
            } else if let Some(text) = matches.get_one::<String>("text") {
                RunSource::Single(SingleSource::Text(text.clone()))
            } else {
                RunSource::Single(SingleSource::Full)
            };
            RunType::Args(RunArgs {
                day: Day { day, year },
                part,
                source,
            })
        } else {
            RunType::Interactive
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RunArgs {
    pub day: Day,
    pub part: PartArgs,
    pub source: RunSource,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PartArgs {
    P1,
    P2,
    Both,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RunSource {
    Single(SingleSource),
    Example(ExampleSource),
}

#[derive(Debug, Error)]
pub enum ReadersError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("no meta entry for day {0}")]
    NoMeta(Day),
}

pub enum SourceReader {
    Simple(Box<dyn BufRead>),
    Example(ExampleReader, Option<Box<dyn BufRead>>),
}

pub enum ExampleReader {
    Single {
        file: Box<dyn BufRead>,
        expected_answer: Answer,
    },
    Multi {
        file_1: Box<dyn BufRead>,
        expected_answer_1: String,
        file_2: Box<dyn BufRead>,
        expected_answer_2: String,
    },
}

impl RunSource {
    pub fn to_readers(self, meta: &DaysMeta, day: Day) -> Result<SourceReader, ReadersError> {
        match self {
            RunSource::Example(example) => {
                let Some(day_info) = meta.get_day(day) else {
                    return Err(ReadersError::NoMeta(day));
                };

                let reader: Result<ExampleReader, ReadersError> = match day_info.example.clone() {
                    Example::Single {
                        path,
                        expected_answer,
                    } => {
                        let file = File::open(path)?;
                        Ok(ExampleReader::Single {
                            file: Box::new(BufReader::new(file)),
                            expected_answer,
                        })
                    }
                    Example::Multi {
                        path_1,
                        expected_answer_1,
                        path_2,
                        expected_answer_2,
                    } => {
                        let file_1 = File::open(path_1)?;
                        let file_2 = File::open(path_2)?;

                        Ok(ExampleReader::Multi {
                            file_1: Box::new(Box::new(BufReader::new(file_1))),
                            expected_answer_1,
                            file_2: Box::new(Box::new(BufReader::new(file_2))),
                            expected_answer_2,
                        })
                    }
                };

                let reader = reader?;

                match example {
                    ExampleSource::ExampleOnly => Ok(SourceReader::Example(reader, None)),
                    ExampleSource::Main => {
                        let file = File::open(&day_info.full)?;
                        let full_reader = Box::new(BufReader::new(file));
                        Ok(SourceReader::Example(reader, Some(full_reader)))
                    }
                }
            }
            RunSource::Single(single) => match single {
                SingleSource::File(file) => {
                    let file = File::open(file)?;
                    Ok(SourceReader::Simple(Box::new(BufReader::new(file))))
                }
                SingleSource::Text(text) => {
                    let cursor = Cursor::new(text);
                    Ok(SourceReader::Simple(Box::new(cursor)))
                }
                SingleSource::Full => {
                    let Some(day_info) = meta.get_day(day) else {
                        return Err(ReadersError::NoMeta(day));
                    };

                    let file = File::open(&day_info.full)?;
                    Ok(SourceReader::Simple(Box::new(BufReader::new(file))))
                }
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum RunGiven {
    #[default]
    Full,
    Example,
    Both,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ExampleSource {
    Main,
    ExampleOnly,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SingleSource {
    Full,
    File(PathBuf), // -f or --file
    Text(String),  // -t or --text
}

#[test]
fn verify_cmd() {
    RunType::command().debug_assert();
}

#[test]
fn should_parse_interactive() {
    // given
    let input: Vec<&str> = vec![];
    let expected = RunType::Interactive;
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_full() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020"];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::Both,
        source: RunSource::Single(SingleSource::Full),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_example() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-e"];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::Both,
        source: RunSource::Example(ExampleSource::ExampleOnly),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_example_full() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-m"];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::Both,
        source: RunSource::Example(ExampleSource::Main),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_file() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-f", "\"lol hi.txt\""];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::Both,
        source: RunSource::Single(SingleSource::File(PathBuf::from("\"lol hi.txt\""))),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_text() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-t", "\"123 123\""];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::Both,
        source: RunSource::Single(SingleSource::Text("\"123 123\"".to_string())),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_p1() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-t", "\"123 123\"", "-1"];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::P1,
        source: RunSource::Single(SingleSource::Text("\"123 123\"".to_string())),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_p2() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-t", "\"123 123\"", "-2"];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::P2,
        source: RunSource::Single(SingleSource::Text("\"123 123\"".to_string())),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}

#[test]
fn should_parse_p12() {
    // given
    let input = vec!["app", "-d", "23", "-y", "2020", "-t", "\"123 123\"", "-12"];
    let expected = RunType::Args(RunArgs {
        day: Day {
            day: 23,
            year: 2020,
        },
        part: PartArgs::Both,
        source: RunSource::Single(SingleSource::Text("\"123 123\"".to_string())),
    });
    // when
    let actual = RunType::parse_from(input);
    // then
    assert_eq!(expected, actual);
}
