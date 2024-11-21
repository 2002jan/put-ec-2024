use std::path::PathBuf;
use std::time::Instant;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution;
use tsp_utils::output_writer::write_output;
use crate::TspAlgorithm;

pub fn test_tsp_algorithm_with_runs<Algorithm: TspAlgorithm>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, output_path: &Option<PathBuf>, verbose: bool, runs: usize) -> (i32, i32, i32) {
    let mut problem_size = runs;

    if problem_size > cost_matrix.size() {
        problem_size = cost_matrix.size();
    }

    let mut min_cost = i32::MAX;
    let mut min_solution = vec![0];
    let mut max_cost = 0;
    let mut max_solution = vec![0];
    let mut aggregated_cost = 0;

    let start = Instant::now();

    for starting_point in 0..problem_size as i32 {
        let solution = Algorithm::run(&cost_matrix, &points_cost, Option::from(starting_point));
        let cost = evaluate_solution(&solution, &cost_matrix, &points_cost);

        if min_cost > cost {
            min_cost = cost;
            min_solution = solution.clone();
        }

        if max_cost < cost {
            max_cost = cost;
            max_solution = solution.clone();
        }

        aggregated_cost += cost;
    }

    let duration = start.elapsed();


    if let Some(path) = output_path {
        let min_output_path = path.join(format!("{}_min_score_output.csv", Algorithm::snaked_name()));
        let max_output_path = path.join(format!("{}_max_score_output.csv", Algorithm::snaked_name()));

        write_output(min_solution.as_ref(), min_output_path.as_ref());
        write_output(max_solution.as_ref(), max_output_path.as_ref());
    }

    let aggregated_cost = aggregated_cost as f32 / problem_size as f32;
    let aggregated_cost = aggregated_cost.round() as i32;

    if verbose {
        println!("Results for {}\nMin cost: {}\nMax cost: {}\nAverage cost: {}\n", Algorithm::name(), min_cost, max_cost, aggregated_cost);

        let duration_micros = duration.as_micros();
        let duration_per_run = duration_micros / (problem_size as u128);

        println!("Time took for {} runs: {:.8}s, time per run: {}μs\n", problem_size, duration.as_secs_f64(), duration_per_run);
        println!("Best solution:\n{:?}\n", min_solution);
    }

    (min_cost, max_cost, aggregated_cost)
}

pub fn test_tsp_algorithm<Algorithm: TspAlgorithm>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, output_path: &Option<PathBuf>, verbose: bool) -> (i32, i32, i32)
{
    let problem_size = cost_matrix.size();
    test_tsp_algorithm_with_runs::<Algorithm>(cost_matrix, points_cost, output_path, verbose, problem_size)
}