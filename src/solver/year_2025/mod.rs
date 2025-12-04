use crate::day::{Day, Solver, SolverDatabase};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub fn add_all(database: &mut SolverDatabase) {
    database.add_solver(Day::new(1, 2025), Solver::separated((day_1::part_1, day_1::part_2)));
    database.add_solver(Day::new(2, 2025), Solver::combined(day_2::solve));
    database.add_solver(Day::new(3, 2025), Solver::combined(day_3::solve));
    database.add_solver(Day::new(4, 2025), Solver::combined(day_4::solve));

}