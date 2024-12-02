use crate::day::{Day, Solver, SolverDatabase};

mod day_1;
mod day_2;

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
    ];

    for (day, solver) in solvers {
        solver_database.add_solver(day, solver);
    }
}