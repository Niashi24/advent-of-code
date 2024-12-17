use crate::day::{Day, Solver, SolverDatabase};

mod day_1;
mod day_10;
mod day_11;
pub mod day_12;
mod day_13;
pub mod day_14;
mod day_15;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_16;
mod day_17;

pub fn add_all(solver_database: &mut SolverDatabase) {
    let solvers = vec![
        (Day::new(1, 2024), Solver::Combined(Box::new(day_1::Day1))),
        (Day::new(2, 2024), Solver::Combined(Box::new(day_2::Day2))),
        (Day::new(3, 2024), Solver::Combined(Box::new(day_3::Day3))),
        (Day::new(4, 2024), Solver::Combined(Box::new(day_4::Day4))),
        (Day::new(5, 2024), Solver::Combined(Box::new(day_5::Day5))),
        (Day::new(6, 2024), Solver::Combined(Box::new(day_6::Day6))),
        (Day::new(7, 2024), Solver::Combined(Box::new(day_7::Day7))),
        (Day::new(8, 2024), Solver::Combined(Box::new(day_8::Day8))),
        (
            Day::new(9, 2024),
            Solver::separated((day_9::part_1, day_9::part_2)),
        ),
        (Day::new(10, 2024), Solver::combined(day_10::solve)),
        (Day::new(11, 2024), Solver::combined(day_11::part_1)),
        (Day::new(12, 2024), Solver::combined(day_12::solve)),
        (Day::new(13, 2024), Solver::combined(day_13::solve)),
        (Day::new(14, 2024), Solver::separated((day_14::part_1, ()))),
        (Day::new(15, 2024), Solver::combined(day_15::part_1)),
        (Day::new(16, 2024), Solver::combined(day_16::solve)),
        (Day::new(17, 2024), Solver::separated((day_17::part_1, day_17::part_2))),
    ];

    for (day, solver) in solvers {
        solver_database.add_solver(day, solver);
    }
}
