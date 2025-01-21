use std::marker::PhantomData;
use tsp_utils::cost_matrix::CostMatrix;
use crate::{StartType, TspAlgorithm};
use crate::StartType::FromSolution;

pub struct CombinedSearch<
    A1: TspAlgorithm,
    A2: TspAlgorithm
> {
    a1: PhantomData<A1>,
    a2: PhantomData<A2>,
}

impl<
    A1: TspAlgorithm,
    A2: TspAlgorithm
> TspAlgorithm for CombinedSearch<A1, A2> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let solution = A1::run(cost_matrix, points_cost, start_from);

        A2::run(cost_matrix, points_cost, FromSolution(
            solution.iter().map(|&x| x as usize).collect()
        ))
    }

    fn name() -> String {
        format!("First {} then {}", A1::name(), A2::name())
    }

    fn snaked_name() -> String {
        format!("first_{}_then_{}", A1::snaked_name(), A2::snaked_name())
    }
}