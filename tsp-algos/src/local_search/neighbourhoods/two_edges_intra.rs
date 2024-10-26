use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::get_neighbouring_indexes;
use crate::local_search::neighbourhoods::LocalSearchNeighbourhood;

pub struct TwoEdgesIntra {}

impl LocalSearchNeighbourhood for TwoEdgesIntra {
    fn evaluate_intra(cost_matrix: &CostMatrix, current_solution: &Vec<usize>, start: usize, target: usize) -> i32 {
        let solution_size = current_solution.len();
        let start_point = current_solution[start];
        let target_point = current_solution[target];
        let (prev_index_start, _) = get_neighbouring_indexes(start, solution_size);
        let (_, next_index_target) = get_neighbouring_indexes(target, solution_size);

        if prev_index_start == target && next_index_target == start {
            return 0;
        }

        //Add new costs
        cost_matrix.get(current_solution[prev_index_start], target_point)
            + cost_matrix.get(start_point, current_solution[next_index_target])
            //Subtract old costs
            - cost_matrix.get(current_solution[prev_index_start], start_point)
            - cost_matrix.get(target_point, current_solution[next_index_target])
    }

    fn apply_intra(start: usize, target: usize, current_solution: &mut Vec<usize>) {
        let mut new_order: Vec<usize> = Vec::with_capacity((target - start + 1) * size_of::<usize>());

        for i in (start..target + 1).rev() {
            new_order.push(current_solution[i]);
        }

        for i in start..target+1 {
            current_solution[i] = new_order[i - start];
        }
    }
}