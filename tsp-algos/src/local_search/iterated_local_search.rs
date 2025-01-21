use std::marker::PhantomData;
use std::mem::size_of;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution_usize;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};
use crate::local_search::search_types::LocalSearchType;
use crate::local_search::starting_solution::StartingSolution;
use crate::{StartType, TspAlgorithm};

const MAX_RUN_TIME: u64 = 2000;
const RANDOM_MOVES_COUNT: i32 = 20;

pub struct IteratedLocalSearch<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> {
    t: PhantomData<T>,
    n: PhantomData<N>,
    ss: PhantomData<SS>,
}

impl<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> IteratedLocalSearch<T, N, SS> {
    fn perturbate_solution(solution: &Vec<usize>, points_cost: &Vec<i32>) -> Vec<usize> {
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

            N::apply_move(&mov, &mut new_solution, &mut free_nodes)
        }

        new_solution
    }
}

impl<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> TspAlgorithm for IteratedLocalSearch<T, N, SS> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let interval = Duration::from_millis(MAX_RUN_TIME);

        let start = Instant::now();

        let mut best_solution = match start_from {
            StartType::FromSolution(solution) => solution,
            _ => SS::get_staring_solution(cost_matrix, points_cost, start_from)
        };
        best_solution = T::run::<N>(cost_matrix, points_cost, best_solution);

        let mut best_cost = evaluate_solution_usize(&best_solution, cost_matrix, points_cost);

        while start.elapsed() < interval {
            let mut new_solution = Self::perturbate_solution(&best_solution, points_cost);
            new_solution = T::run::<N>(cost_matrix, points_cost, new_solution);
            let new_cost = evaluate_solution_usize(&new_solution, cost_matrix, points_cost);

            if new_cost < best_cost {
                best_cost = new_cost;
                best_solution = new_solution;
            }
        }

        best_solution.iter().map(|&x| x as i32).collect()
    }

    fn name() -> String {
        SS::name() + " " + N::name().as_str() + " " + T::name().as_str() + " " + "Iterated Local Search"
    }

    fn snaked_name() -> String {
        SS::snaked_name() + "_" + N::snaked_name().as_str() + "_" + T::snaked_name().as_str() + "_" + "iterated_local_search"
    }
}
