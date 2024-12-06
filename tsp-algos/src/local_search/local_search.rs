use std::marker::PhantomData;
use tsp_utils::cost_matrix::CostMatrix;
use crate::local_search::neighbourhoods::LocalSearchNeighbourhood;
use crate::local_search::search_types::LocalSearchType;
use crate::local_search::starting_solution::StartingSolution;
use crate::{StartType, TspAlgorithm};

pub struct LocalSearch<
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
> TspAlgorithm for LocalSearch<T, N, SS> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let staring_solution = SS::get_staring_solution(cost_matrix, points_cost, start_from);

        T::run::<N>(cost_matrix, points_cost, staring_solution).iter().map(|&x| x as i32).collect()
    }

    fn name() -> String {
        SS::name() + " " + N::name().as_str() + " " + T::name().as_str() + " " + "Local Search"
    }

    fn snaked_name() -> String {
        SS::snaked_name() + "_" + N::snaked_name().as_str() + "_" + T::snaked_name().as_str() + "_" + "local_search"
    }
}
