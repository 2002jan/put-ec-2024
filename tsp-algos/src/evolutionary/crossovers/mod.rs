pub mod keep_common_fill_ls_crossover;

pub mod keep_common_fill_random_crossover;

use tsp_utils::cost_matrix::CostMatrix;

pub trait Crossover {
    fn crossover(parent1: &Vec<usize>,parent2: &Vec<usize>, cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<usize>;

    fn name() -> String;

    fn snaked_name() -> String;
}