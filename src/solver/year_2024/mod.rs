use crate::day::{Day, Solver, SolverDatabase};

mod day_1;

pub fn add_all(solver_database: &mut SolverDatabase) {
    let solvers = vec![
        (
            Day::new(1, 2021),
            Solver::Separated(Box::new(day_1::Day1)),
        ),
    ];

    for (day, solver) in solvers {
        solver_database.add_solver(day, solver);
    }
}