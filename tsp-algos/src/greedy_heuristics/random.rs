use rand::seq::SliceRandom;
use rand::thread_rng;
use tsp_utils::cost_matrix::CostMatrix;
use crate::{StartType, TspAlgorithm};

pub struct RandomAlgorithm {}

impl TspAlgorithm for RandomAlgorithm {
    fn run(cost_matrix: &CostMatrix, _points_cost: &Vec<i32>, _start_from: StartType) -> Vec<i32> {
        let size = cost_matrix.size() as i32;
        let solution_size = ((size as f32) / 2.).ceil() as i32;

        let mut solution = (0..cost_matrix.size() as i32).collect::<Vec<_>>();

        solution.shuffle(&mut thread_rng());

        let solution: Vec<i32> = solution[0..solution_size as usize].to_vec();

        solution
    }

    fn name() -> String {
        String::from("Random Algorithm")
    }

    fn snaked_name() -> String {
        String::from("random_algorithm")
    }
}