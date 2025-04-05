#![feature(portable_simd)]

mod solver;

pub fn solve(hands: &Vec<String>, board: &String) -> f32 {
    let solution = solver::Solver::new();
    solution.solve(&hands, &board)
}
