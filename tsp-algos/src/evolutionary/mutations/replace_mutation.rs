use std::mem::size_of;
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::evolutionary::mutations::Mutation;

pub struct ReplaceMutation {}

impl Mutation for ReplaceMutation {
    fn mutate(solution: &Vec<usize>, _cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<usize> {
        let mut new_solution = solution.clone();

        let mut free_nodes = Vec::with_capacity((points_cost.len() - new_solution.len()) * size_of::<usize>());

        for i in 0..points_cost.len() {
            if !new_solution.contains(&i) {
                free_nodes.push(i);
            }
        }

        let mut random = thread_rng();

        let node_to_replace = random.gen_range(0..new_solution.len());
        let node_to_replace_with = random.gen_range(0..free_nodes.len());

        new_solution[node_to_replace] = free_nodes[node_to_replace_with];

        new_solution
    }

    fn name() -> String {
        String::from("Random Replace Mutation")
    }

    fn snaked_name() -> String {
        String::from("random_replace_mutation")
    }
}