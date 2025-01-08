use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;

pub mod greedy_heuristics;
pub mod test_algorithm;
pub mod local_search;
pub mod other;
pub mod evolutionary;

pub trait TspAlgorithm {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32>;

    fn name() -> String;

    fn snaked_name() -> String;
}

#[derive(Clone)]
pub enum StartType {
    FromPoint(usize),
    FromSolution(Vec<usize>),
    FromStart,
}

impl StartType {
    fn get_starting_solution(self, solution_size: usize) -> Vec<usize> {
        match self {
            StartType::FromPoint(point) => {
                let mut solution: Vec<usize> = Vec::with_capacity(solution_size * size_of::<usize>());
                solution.push(point);
                solution
            }
            StartType::FromSolution(solution) => solution,
            StartType::FromStart => {
                let mut solution: Vec<usize> = Vec::with_capacity(solution_size * size_of::<usize>());
                solution.push(0);
                solution
            }
        }
    }
}
