use tsp_utils::cost_matrix::CostMatrix;

pub mod greedy_starting_solution;
pub mod random_starting_solution;

pub trait StartingSolution {
    fn get_staring_solution(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<usize>;

    fn name() -> String;
    fn snaked_name() -> String;
}