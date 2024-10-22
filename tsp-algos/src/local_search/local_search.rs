use tsp_utils::cost_matrix::CostMatrix;
use crate::TspAlgorithm;

pub struct LocalSearch<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> {}

impl<
    T: LocalSearchType,
    N: LocalSearchNeighbourhood,
    SS: StartingSolution
> TspAlgorithm for LocalSearch<T, N, SS> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<i32> {

        let staring_solution = SS::get_staring_solution(cost_matrix, points_cost, start_from);

        vec![0]
    }

    fn name() -> &'static str {
        todo!()
    }

    fn snaked_name() -> &'static str {
        todo!()
    }
}

pub trait LocalSearchType {}

pub trait LocalSearchNeighbourhood {

}

pub trait StartingSolution {
    fn get_staring_solution(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<i32>;
}