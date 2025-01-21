use std::mem::size_of;
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::evolutionary::mutations::Mutation;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};
use crate::local_search::neighbourhoods::two_edges_intra::TwoEdgesIntra;

const RANDOM_MOVES_COUNT: i32 = 1;

pub struct RandomMoveMutation {
}

impl Mutation for RandomMoveMutation {
    fn mutate(solution: &Vec<usize>, _cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<usize> {
        let solution_size = solution.len();
        let free_nodes_size = points_cost.len() - solution_size;
        let mut new_solution = solution.clone();

        let mut free_nodes = Vec::with_capacity(free_nodes_size * size_of::<usize>());

        for i in 0..points_cost.len() {
            if !new_solution.contains(&i) {
                free_nodes.push(i);
            }
        }

        let mut rand = thread_rng();

        for _ in 0..RANDOM_MOVES_COUNT {
            let mov: LocalSearchMove = match rand.gen_bool(0.5) {
                true => {
                    let start = rand.gen_range(0..solution_size);
                    let target = rand.gen_range(0..free_nodes_size);

                    Inter(start, target)
                }
                false => {
                    let start = rand.gen_range(0..solution_size);
                    let mut target = rand.gen_range(0..solution_size - 1);

                    if target >= start {
                        target += 1;
                    }

                    Intra(start, target)
                }
            };

            TwoEdgesIntra::apply_move(&mov, &mut new_solution, &mut free_nodes)
        }

        new_solution
    }

    fn name() -> String {
        String::from("Random Move Mutation")
    }

    fn snaked_name() -> String {
        String::from("random_move_mutation")
    }
}