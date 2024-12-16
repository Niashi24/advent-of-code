use crate::day::{Day, Solver, SolverDatabase};

pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

pub fn add_all(solver_database: &mut SolverDatabase) {
    let solvers = vec![
        (Day::new(1, 2021), Solver::Separated(Box::new(day_1::Day1))),
        (Day::new(2, 2021), Solver::Separated(Box::new(day_2::Day2))),
        (Day::new(3, 2021), Solver::Combined(Box::new(day_3::Day3))),
        (Day::new(4, 2021), Solver::Combined(Box::new(day_4::Day4))),
        (Day::new(5, 2021), Solver::Combined(Box::new(day_5::Day5))),
        (Day::new(6, 2021), Solver::Combined(Box::new(day_6::Day6))),
        (Day::new(7, 2021), Solver::Combined(Box::new(day_7::Day7))),
        (Day::new(8, 2021), Solver::Separated(Box::new(day_8::Day8))),
        (Day::new(9, 2021), Solver::Combined(Box::new(day_9::Day9))),
        (
            Day::new(10, 2021),
            Solver::Separated(Box::new(day_10::Day10)),
        ),
        (
            Day::new(11, 2021),
            Solver::Separated(Box::new(day_11::Day11)),
        ),
        (
            Day::new(12, 2021),
            Solver::Combined(Box::new(day_12::Day12)),
        ),
        (
            Day::new(13, 2021),
            Solver::Combined(Box::new(day_13::Day13)),
        ),
        (
            Day::new(14, 2021),
            Solver::Combined(Box::new(day_14::Day14)),
        ),
        (
            Day::new(15, 2021),
            Solver::Combined(Box::new(day_15::Day15)),
        ),
        (
            Day::new(16, 2021),
            Solver::Combined(Box::new(day_16::Day16)),
        ),
        (
            Day::new(17, 2021),
            Solver::Separated(Box::new(day_17::Day17)),
        ),
        (
            Day::new(18, 2021),
            Solver::Separated(Box::new(day_18::Day18)),
        ),
        (
            Day::new(19, 2021),
            Solver::Combined(Box::new(day_19::Day19)),
        ),
        (
            Day::new(20, 2021),
            Solver::Combined(Box::new(day_20::Day20)),
        ),
        (
            Day::new(21, 2021),
            Solver::Combined(Box::new(day_21::Day21)),
        ),
        (
            Day::new(22, 2021),
            Solver::Combined(Box::new(day_22::Day22)),
        ),
        (
            Day::new(23, 2021),
            Solver::Combined(Box::new(day_23::Day23)),
        ),
        (
            Day::new(24, 2021),
            Solver::Combined(Box::new(day_24::Day24)),
        ),
        (
            Day::new(25, 2021),
            Solver::Combined(Box::new(day_25::Day25)),
        ),
    ];

    for (day, solver) in solvers {
        solver_database.add_solver(day, solver);
    }
}
