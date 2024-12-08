use crate::day::{Day, Solver, SolverDatabase};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

pub fn add_all(solver_database: &mut SolverDatabase) {
    let solvers = vec![
        (
            Day::new(1, 2024),
            Solver::Combined(Box::new(day_1::Day1)),
        ),
        (
            Day::new(2, 2024),
            Solver::Combined(Box::new(day_2::Day2)),
        ),
        (
            Day::new(3, 2024),
            Solver::Combined(Box::new(day_3::Day3)),
        ),
        (
            Day::new(4, 2024),
            Solver::Combined(Box::new(day_4::Day4)),
        ),
        (
            Day::new(5, 2024),
            Solver::Combined(Box::new(day_5::Day5)),
        ),
        (
            Day::new(6, 2024),
            Solver::Combined(Box::new(day_6::Day6)),
        ),
        (
            Day::new(7, 2024),
            Solver::Combined(Box::new(day_7::Day7)),
        ),
        (
            Day::new(8, 2024),
            Solver::Combined(Box::new(day_8::Day8)),
        ),
    ];

    for (day, solver) in solvers {
        solver_database.add_solver(day, solver);
    }
}