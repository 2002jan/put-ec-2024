use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};
use crate::local_search::search_types::LocalSearchType;

pub struct SteepestLocalSearch {
    current_start: usize,
    next_start: usize,
    next_intra_target: usize,
    next_inter_target: usize,
    intra_size: usize,
    inter_size: usize,
}


impl LocalSearchType for SteepestLocalSearch {
    fn new(solution_size: usize, free_nodes_size: usize) -> impl LocalSearchType {
        SteepestLocalSearch {
            current_start: 0,
            next_start: 0,
            next_intra_target: 0,
            next_inter_target: 0,
            intra_size: solution_size,
            inter_size: free_nodes_size,
        }
    }

    fn run<N: LocalSearchNeighbourhood>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, starting_solution: Vec<usize>) -> Vec<usize> {
        let mut current_solution = starting_solution;
        let mut free_nodes = Vec::with_capacity((points_cost.len() - current_solution.len()) * size_of::<usize>());

        for i in 0..points_cost.len() {
            if !current_solution.contains(&i) {
                free_nodes.push(i);
            }
        }

        let solution_size = current_solution.len();
        let free_nodes_size = free_nodes.len();

        loop {
            let mut neighbourhood_interator = Self::new(solution_size, free_nodes_size);

            let mut best_change = 0;
            let mut bets_move: Option<LocalSearchMove> = None;

            loop {
                let next_move = neighbourhood_interator.next();


                let next_move = match next_move {
                    None => {
                        break;
                    }
                    Some(mov) => mov
                };

                let change = N::evaluate_move(cost_matrix, points_cost, &next_move, &current_solution, &free_nodes);

                if change < best_change {
                    best_change = change;
                    bets_move = Some(next_move);
                }
            }

            if let Some(mov) = bets_move {
                N::apply_move(&mov, &mut current_solution, &mut free_nodes);
            } else {
                break;
            }
        }

        current_solution
    }

    fn next(&mut self) -> Option<LocalSearchMove> {
        if (self.next_inter_target >= self.inter_size && self.next_inter_target >= self.inter_size) || self.next_start == 0 {
            self.current_start = self.next_start;
            self.next_start += 1;

            self.next_intra_target = self.next_start + 1;
            self.next_inter_target = 0;
        }

        if self.next_start >= self.intra_size {
            return None;
        }

        if self.next_intra_target < self.intra_size {
            self.next_intra_target += 1;
            Some(Intra(self.current_start, self.next_intra_target - 1))
        } else {
            self.next_inter_target += 1;
            Some(Inter(self.current_start, self.next_inter_target - 1))
        }
    }
}
