use rand::seq::SliceRandom;
use tsp_utils::cost_matrix::CostMatrix;
use rand::thread_rng;
use crate::local_search::local_search::StartingSolution;

pub struct RandomStartingSolution;

impl StartingSolution for RandomStartingSolution {
    fn get_staring_solution(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<i32> {
        let size = cost_matrix.size();
        let solution_size = ((size as f32) / 2.).ceil() as i32;
        let mut nodes: Vec<i32> = (0..solution_size as i32).collect::<Vec<i32>>();
        nodes.shuffle(&mut thread_rng());

        nodes
    }
}