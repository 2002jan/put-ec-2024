use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution;
use crate::TspAlgorithm;

pub fn test_tsp_algorithm<Algorithm: TspAlgorithm>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, verbose: bool) -> (i32, i32, i32)
{
    let problem_size = cost_matrix.size();
    let mut min_cost = i32::MAX;
    let mut max_cost = 0;
    let mut aggregated_cost = 0;

    for starting_point in 0..problem_size as i32 {
        let solution = Algorithm::run(&cost_matrix, &points_cost, Option::from(starting_point));
        let cost = evaluate_solution(&solution, &cost_matrix, &points_cost);

        if min_cost > cost {
            min_cost = cost;
        }

        if max_cost < cost {
            max_cost = cost;
        }

        aggregated_cost += cost;
    }

    let aggregated_cost = aggregated_cost as f32 / problem_size as f32;
    let aggregated_cost = aggregated_cost.round() as i32;

    if verbose {
        println!("Results for {}\nMin cost: {}\nMax cost: {}\nAverage cost: {}\n", Algorithm::name(), min_cost, max_cost, aggregated_cost);
    }

    (min_cost, max_cost, aggregated_cost)
}