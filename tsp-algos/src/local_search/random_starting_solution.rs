use rand::seq::SliceRandom;
use tsp_utils::cost_matrix::CostMatrix;
use rand::thread_rng;
use crate::local_search::local_search::StartingSolution;

pub struct RandomStartingSolution;

impl StartingSolution for RandomStartingSolution {
    fn get_staring_solution(cost_matrix: &CostMatrix, _: &Vec<i32>, _: Option<i32>) -> Vec<usize> {
        let size = cost_matrix.size();
        let solution_size = ((size as f32) / 2.).ceil() as i32;
        let mut nodes: Vec<i32> = (0..solution_size).collect::<Vec<i32>>();
        nodes.shuffle(&mut thread_rng());

        nodes.iter().map(|&x| x as usize).collect::<Vec<usize>>()
    }

    fn name() -> String {
        String::from("Random Start")
    }

    fn snaked_name() -> String {
        String::from("random_start")
    }
}