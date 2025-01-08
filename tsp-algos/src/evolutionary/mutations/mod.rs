use tsp_utils::cost_matrix::CostMatrix;

pub mod replace_mutation;

pub trait Mutation {
    fn mutate(solution: &Vec<usize>, cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<usize>;

    fn name() -> String;

    fn snaked_name() -> String;
}