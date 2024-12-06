use std::marker::PhantomData;
use std::time::{Duration, Instant};
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution_usize;
use crate::local_search::search_types::LocalSearchType;
use crate::other::destroy_methods::SolutionDestroyer;
use crate::{StartType, TspAlgorithm};
use crate::local_search::neighbourhoods::two_edges_intra::TwoEdgesIntra;
use crate::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use crate::local_search::starting_solution::StartingSolution;

const MAX_RUN_TIME: u64 = 2;

pub struct LargeNeighborhoodSearchWith<
    T: LocalSearchType,
    R: TspAlgorithm,
    D: SolutionDestroyer
> {
    t: PhantomData<T>,
    r: PhantomData<R>,
    d: PhantomData<D>,
}

impl<
    T: LocalSearchType,
    R: TspAlgorithm,
    D: SolutionDestroyer
> TspAlgorithm for LargeNeighborhoodSearchWith<T, R, D> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let interval = Duration::from_secs(MAX_RUN_TIME);
        let start = Instant::now();

        let mut best_solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, start_from);

        best_solution = T::run::<TwoEdgesIntra>(cost_matrix, points_cost, best_solution);

        let mut best_cost = evaluate_solution_usize(&best_solution, cost_matrix, points_cost);

        while start.elapsed() < interval {
            let mut new_solution = best_solution.clone();

            D::destroy(&mut new_solution, cost_matrix, points_cost);
            new_solution = R::run(
                cost_matrix,
                points_cost,
                StartType::FromSolution(new_solution),
            ).iter().map(|&x| x as usize).collect();

            new_solution = T::run::<TwoEdgesIntra>(cost_matrix, points_cost, new_solution);

            let new_cost = evaluate_solution_usize(&new_solution, cost_matrix, points_cost);

            if new_cost < best_cost {
                best_cost = new_cost;
                best_solution = new_solution;
            }
        }

        best_solution.iter().map(|&x| x as i32).collect()
    }

    fn name() -> String {
        R::name() + " " + D::name().as_str() + " " + "LNSw"
    }

    fn snaked_name() -> String {
        R::snaked_name() + "_" + D::snaked_name().as_str() + "_" + "lns_w"
    }
}

pub struct LargeNeighborhoodSearch<
    R: TspAlgorithm,
    D: SolutionDestroyer
> {
    r: PhantomData<R>,
    d: PhantomData<D>,
}

impl<
    R: TspAlgorithm,
    D: SolutionDestroyer
> TspAlgorithm for LargeNeighborhoodSearch<R, D> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let interval = Duration::from_secs(MAX_RUN_TIME);
        let start = Instant::now();

        let mut best_solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, start_from);
        let mut best_cost = evaluate_solution_usize(&best_solution, cost_matrix, points_cost);

        while start.elapsed() < interval {
            let mut new_solution = best_solution.clone();

            D::destroy(&mut new_solution, cost_matrix, points_cost);
            new_solution = R::run(
                cost_matrix,
                points_cost,
                StartType::FromSolution(new_solution),
            ).iter().map(|&x| x as usize).collect();

            let new_cost = evaluate_solution_usize(&new_solution, cost_matrix, points_cost);

            if new_cost < best_cost {
                best_cost = new_cost;
                best_solution = new_solution;
            }
        }

        best_solution.iter().map(|&x| x as i32).collect()
    }

    fn name() -> String {
        R::name() + " " + D::name().as_str() + " " + "LNS"
    }

    fn snaked_name() -> String {
        R::snaked_name() + "_" + D::snaked_name().as_str() + "_" + "lns"
    }
}

