use std::marker::PhantomData;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution;
use crate::local_search::starting_solution::StartingSolution;
use crate::local_search::neighbourhoods::LocalSearchNeighbourhood;
use crate::local_search::search_types::LocalSearchType;
use crate::{StartType, TspAlgorithm};

const ITERATIONS: i32 = 200;

pub struct MultipleStartLocalSearch<
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
> TspAlgorithm for MultipleStartLocalSearch<T, N, SS> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let mut best_solution: Vec<i32> = vec![];
        let mut best_score = i32::MAX;

        for _ in 0..ITERATIONS {
            let staring_solution = SS::get_staring_solution(cost_matrix, points_cost, start_from.clone());

            let solution = T::run::<N>(cost_matrix, points_cost, staring_solution).iter().map(|&x| x as i32).collect::<Vec<i32>>();
            let solution_score = evaluate_solution(&solution, cost_matrix, points_cost);

            if solution_score < best_score {
                best_solution = solution;
                best_score = solution_score;
            }
        }

        best_solution
    }

    fn name() -> String {
        SS::name() + " " + N::name().as_str() + " " + T::name().as_str() + " " + "Multiple Start Local Search"
    }

    fn snaked_name() -> String {
        SS::snaked_name() + "_" + N::snaked_name().as_str() + "_" + T::snaked_name().as_str() + "_" + "multiple_start_local_search"
    }
}