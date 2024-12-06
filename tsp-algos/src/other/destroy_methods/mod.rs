pub mod random_destroy;

use tsp_utils::cost_matrix::CostMatrix;

pub trait SolutionDestroyer {
    fn destroy(solution: &mut Vec<usize>, cost_matrix: &CostMatrix, points_cost: &Vec<i32>);

    fn name() -> String;
    fn snaked_name() -> String;
}