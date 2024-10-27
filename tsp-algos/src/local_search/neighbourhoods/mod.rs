pub mod two_nodes_intra;
pub mod two_edges_intra;

use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::get_neighbouring_indexes;

pub enum LocalSearchMove {
    Intra(usize, usize),
    Inter(usize, usize),
}

pub trait LocalSearchNeighbourhood {
    fn evaluate_move(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, mov: &LocalSearchMove, current_solution: &Vec<usize>, free_nodes: &Vec<usize>) -> i32 {
        match mov {
            LocalSearchMove::Intra(start, target) => Self::evaluate_intra(cost_matrix, current_solution, *start, *target),
            LocalSearchMove::Inter(start, target) => Self::evaluate_inter(cost_matrix, points_cost, current_solution, free_nodes, *start, *target),
        }
    }

    fn apply_move(mov: &LocalSearchMove, current_solution: &mut Vec<usize>, free_nodes: &mut Vec<usize>) {
        match mov {
            LocalSearchMove::Intra(start, target) => Self::apply_intra(*start, *target, current_solution),
            LocalSearchMove::Inter(start, target) => Self::apply_inter(*start, *target, current_solution, free_nodes)
        }
    }

    fn evaluate_intra(cost_matrix: &CostMatrix, current_solution: &Vec<usize>, start: usize, target: usize) -> i32;

    fn apply_intra(start: usize, target: usize, current_solution: &mut Vec<usize>);

    fn evaluate_inter(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, current_solution: &Vec<usize>, free_nodes: &Vec<usize>, start: usize, target: usize) -> i32 {
        let current_node = current_solution[start];
        let new_node = free_nodes[target];

        let (prev_index, next_index) = get_neighbouring_indexes(start, current_solution.len());

        points_cost[new_node] - points_cost[current_node]
            + cost_matrix.get(new_node, current_solution[next_index])
            + cost_matrix.get(current_solution[prev_index], new_node)
            - cost_matrix.get(current_node, current_solution[next_index])
            - cost_matrix.get(current_solution[prev_index], current_node)
    }

    fn apply_inter(start: usize, target: usize, current_solution: &mut Vec<usize>, free_nodes: &mut Vec<usize>) {
        let buffer = current_solution[start];
        current_solution[start] = free_nodes[target];
        free_nodes[target] = buffer;
    }

    fn name() -> String;

    fn snaked_name() -> String;
}