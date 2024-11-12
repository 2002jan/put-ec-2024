use std::collections::LinkedList;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::get_neighbouring_indexes;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};

pub struct TwoNodesIntra {}

impl LocalSearchNeighbourhood for TwoNodesIntra {
    fn evaluate_intra(cost_matrix: &CostMatrix, current_solution: &Vec<usize>, start: usize, target: usize) -> i32 {
        let solution_size = current_solution.len();
        let start_point = current_solution[start];
        let target_point = current_solution[target];
        let (prev_index_start, next_index_start) = get_neighbouring_indexes(start, solution_size);
        let (prev_index_target, next_index_target) = get_neighbouring_indexes(target, solution_size);

        if start + 1 == target {
            //Add new costs
            cost_matrix.get(current_solution[prev_index_start], target_point)
                + cost_matrix.get(start_point, current_solution[next_index_target])
                //Subtract old costs
                - cost_matrix.get(current_solution[prev_index_start], start_point)
                - cost_matrix.get(target_point, current_solution[next_index_target])
        } else if start == 0 && target == solution_size - 1 {
            //Add new costs
            cost_matrix.get(current_solution[prev_index_target], start_point)
                + cost_matrix.get(target_point, current_solution[next_index_start])
                //Subtract old costs
                - cost_matrix.get(current_solution[prev_index_target], target_point)
                - cost_matrix.get(start_point, current_solution[next_index_start])
        } else {
            //Add new costs
            cost_matrix.get(current_solution[prev_index_start], target_point)
                + cost_matrix.get(target_point, current_solution[next_index_start])
                + cost_matrix.get(current_solution[prev_index_target], start_point)
                + cost_matrix.get(start_point, current_solution[next_index_target])
                //Subtract old costs
                - cost_matrix.get(current_solution[prev_index_start], start_point)
                - cost_matrix.get(start_point, current_solution[next_index_start])
                - cost_matrix.get(current_solution[prev_index_target], target_point)
                - cost_matrix.get(target_point, current_solution[next_index_target])
        }

    }

    fn apply_intra(start: usize, target: usize, current_solution: &mut Vec<usize>) {
        let buffer = current_solution[start];
        current_solution[start] = current_solution[target];
        current_solution[target] = buffer;
    }

    fn get_new_moves_intra(_start: usize, _target: usize, _solution_size: usize, _free_nodes_size: usize) -> LinkedList<LocalSearchMove> {
        panic!("Not implemented yet (never will be)")
    }

    fn name() -> String {
        String::from("Two Nodes Intra")
    }

    fn snaked_name() -> String {
        String::from("two_nodes_intra")
    }
}
