use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::other::destroy_methods::SolutionDestroyer;

const DESTROY_PERCENTAGE: f32 = 0.3;

pub struct RandomDestroy {

}

impl SolutionDestroyer for RandomDestroy {
    fn destroy(solution: &mut Vec<usize>, _cost_matrix: &CostMatrix, _points_cost: &Vec<i32>) {
        let mut rng = thread_rng();

        let solution_size = solution.len();
        let edges_to_remove = solution_size as f32 * DESTROY_PERCENTAGE;
        let edges_to_remove = edges_to_remove.ceil() as usize;

        let target_size = solution_size - edges_to_remove;

        while solution.len() > target_size {
            let node_to_remove = rng.gen_range(0..solution.len());

            solution.remove(node_to_remove);
        }
    }

    fn name() -> String {
        String::from("Random Destroy")
    }

    fn snaked_name() -> String {
        String::from("random_destroy")
    }
}
