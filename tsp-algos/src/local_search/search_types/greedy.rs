use std::mem::size_of;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};
use crate::local_search::search_types::{LocalSearchType};

pub struct GreedyLocalSearch {
    current_start: usize,
    next_start: usize,
    next_intra_target: usize,
    next_inter_target: usize,
    random_thread: ThreadRng,
    start_order: Vec<usize>,
    intra_targets: Vec<usize>,
    inter_targets: Vec<usize>,
    intra_size: usize,
    inter_size: usize,
}

impl LocalSearchType for GreedyLocalSearch {
    fn new(solution_size: usize, free_nodes_size: usize) -> impl LocalSearchType {
        let mut thread_rng = thread_rng();

        let mut start_order = (0..solution_size).collect::<Vec<usize>>();
        start_order.shuffle(&mut thread_rng);

        let intra_targets = (0..solution_size).collect::<Vec<usize>>();
        let inter_targets = (0..free_nodes_size).collect::<Vec<usize>>();

        GreedyLocalSearch {
            current_start: 0,
            next_start: 0,
            next_intra_target: 0,
            next_inter_target: 0,
            random_thread: thread_rng,
            start_order,
            intra_targets,
            inter_targets,
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
            let mut found_better = false;
            let mut neighbourhood_interator = Self::new(solution_size, free_nodes_size);

            loop {
                let next_move = neighbourhood_interator.next();


                let next_move = match next_move {
                    None => {
                        break;
                    }
                    Some(mov) => mov
                };

                let change = N::evaluate_move(cost_matrix, points_cost, &next_move, &current_solution, &free_nodes);

                if change < 0 {
                    N::apply_move(&next_move, &mut current_solution, &mut free_nodes);

                    found_better = true;
                    break;
                }
            }

            if !found_better {
                break;
            }
        }

        current_solution
    }

    fn next(&mut self) -> Option<LocalSearchMove> {
        if (self.next_inter_target >= self.inter_size && self.next_inter_target >= self.inter_size) || self.next_start == 0 {
            self.current_start = self.next_start;
            self.next_start += 1;

            self.next_intra_target = 0;
            self.next_inter_target = 0;

            self.intra_targets.shuffle(&mut self.random_thread);
            self.inter_targets.shuffle(&mut self.random_thread);
        }

        if self.next_start >= self.start_order.len() {
            return None;
        }

        let current_start = self.start_order[self.current_start];

        let next_move_intra: bool;

        if self.next_inter_target >= self.inter_size && self.next_intra_target < self.intra_size {
            next_move_intra = true;
        } else if self.next_inter_target < self.inter_size && self.next_intra_target >= self.intra_size {
            next_move_intra = false;
        } else {
            next_move_intra = self.random_thread.gen_bool(0.5)
        }

        match next_move_intra {
            true => {
                let mut current_target = self.intra_targets[self.next_intra_target];
                self.next_intra_target += 1;

                while current_target <= current_start {
                    if self.next_intra_target >= self.intra_size {
                        return if self.next_inter_target < self.inter_size {
                            let current_target = self.inter_targets[self.next_inter_target];
                            self.next_inter_target += 1;

                            Some(Inter(current_start, current_target))
                        } else {
                            None
                        };
                    }

                    current_target = self.intra_targets[self.next_intra_target];
                    self.next_intra_target += 1;
                }

                Some(Intra(current_start, current_target))
            }
            false => {
                let current_target = self.inter_targets[self.next_inter_target];
                self.next_inter_target += 1;

                Some(Inter(current_start, current_target))
            }
        }
    }
}