use tsp_utils::cost_matrix::CostMatrix;

pub mod greedy_heuristics;
pub mod test_algorithm;
pub mod local_search;

pub trait TspAlgorithm {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<i32>;

    fn name() -> &'static str;

    fn snaked_name() -> &'static str;
}