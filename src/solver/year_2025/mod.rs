use crate::day::{Day, Solver, SolverDatabase};

mod day_1;

pub fn add_all(database: &mut SolverDatabase) {
    database.add_solver(Day::new(1, 2025), Solver::separated((day_1::part_1, day_1::part_2)));
}