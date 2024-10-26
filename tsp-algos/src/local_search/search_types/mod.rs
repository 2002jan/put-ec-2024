use tsp_utils::cost_matrix::CostMatrix;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};

pub mod greedy;
pub mod steepest;

pub trait LocalSearchType {
    fn new(solution_size: usize, free_nodes_size: usize) -> impl LocalSearchType;
    fn run<N: LocalSearchNeighbourhood> (cost_matrix: &CostMatrix, points_cost: &Vec<i32>, starting_solution: Vec<usize>) -> Vec<usize>;

    fn next(&mut self) -> Option<LocalSearchMove>;
}