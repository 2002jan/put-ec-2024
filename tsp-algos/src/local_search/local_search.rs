use std::marker::PhantomData;
use tsp_utils::cost_matrix::CostMatrix;
use crate::local_search::neighbourhoods::LocalSearchNeighbourhood;
use crate::local_search::search_types::{LocalSearchType};
use crate::TspAlgorithm;

pub struct LocalSearch<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> {
    t: PhantomData<T>,
    n: PhantomData<N>,
    ss: PhantomData<SS>
}

impl<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> TspAlgorithm for LocalSearch<T, N, SS> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<i32> {

        let staring_solution = SS::get_staring_solution(cost_matrix, points_cost, start_from);

        T::run::<N>(cost_matrix, points_cost, staring_solution).iter().map(|&x| x as i32).collect()
    }

    fn name() -> &'static str {
        "Local Search"
    }

    fn snaked_name() -> &'static str {
        "local_search"
    }
}


pub trait StartingSolution {
    fn get_staring_solution(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<usize>;
}