use std::collections::LinkedList;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::get_neighbouring_indexes;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};

pub struct TwoEdgesIntra {}

impl LocalSearchNeighbourhood for TwoEdgesIntra {
    fn evaluate_intra(cost_matrix: &CostMatrix, current_solution: &Vec<usize>, start: usize, target: usize) -> i32 {
        if start == target {
            return 0;
        }

        let solution_size = current_solution.len();
        let start_point = current_solution[start];
        let target_point = current_solution[target];
        let (prev_index_start, _) = get_neighbouring_indexes(start, solution_size);
        let (_, next_index_target) = get_neighbouring_indexes(target, solution_size);


        if prev_index_start == target {
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
        let solution_size = current_solution.len();

        let mut new_order: LinkedList<usize> = LinkedList::new();

        let mut i = start;

        loop {
            new_order.push_back(current_solution[i]);

            if i == target {
                break;
            }

            i = (i + 1) % solution_size;
        }

        i = start;

        loop {
            current_solution[i] = new_order.pop_back().unwrap();

            if i == target {
                break;
            }

            i = (i + 1) % solution_size;
        }
    }

    fn get_new_moves_intra(start: usize, target: usize, solution_size: usize, free_nodes_size: usize) -> LinkedList<LocalSearchMove> {
        let mut new_moves: LinkedList<LocalSearchMove> = LinkedList::new();

        let (start_prev, _) = get_neighbouring_indexes(start, solution_size);
        let (_, target_next) = get_neighbouring_indexes(target, solution_size);

        for i in 0..free_nodes_size {
            new_moves.push_back(Inter(start_prev, i));
            new_moves.push_back(Inter(start, i));
            new_moves.push_back(Inter(target, i));
            new_moves.push_back(Inter(target_next, i));
        }

        for i in 0..solution_size {
            if i != start_prev {
                new_moves.push_back(Intra(i, start_prev));
            }

            if i != target_next {
                new_moves.push_back(Intra(target_next, i));
            }

            if i != start {
                new_moves.push_back(Intra(start, i));
                new_moves.push_back(Intra(i, start));
            }

            if i != target {
                new_moves.push_back(Intra(target, i));
                new_moves.push_back(Intra(i, target));
            }
        }

        new_moves
    }

    fn name() -> String {
        String::from("Two Edges Intra")
    }

    fn snaked_name() -> String {
        String::from("two_edges_intra")
    }
}