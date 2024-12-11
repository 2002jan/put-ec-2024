use std::collections::HashMap;
use std::path::PathBuf;
use rand::{thread_rng, Rng};
use tsp_algos::TspAlgorithm;
use tsp_algos::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use tsp_algos::local_search::starting_solution::StartingSolution;
use tsp_algos::StartType::{FromPoint, FromSolution};
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution;
pub trait SimMeasure {
    fn evaluate_similarity(solution1: &Vec<i32>, solution2: &Vec<i32>) -> i32;
    fn get_name() -> String;
    fn get_snaked_name() -> String;
}
pub fn check_similarity_best<
    BestAlgorithm: TspAlgorithm,
    GreedyAlgorithm: TspAlgorithm,
    SimilarityMeasure: SimMeasure
>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, output_path: &Option<PathBuf>, verbose: bool) -> Vec<(i32, i32)> {
    //     use the iterated local search to generate initial solution
    //     get random starting point
    let mut rng = thread_rng();
    let start = rng.gen_range(0..cost_matrix.size());
    let start_from = FromPoint(start);


    let initial_random_solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, start_from);
    let best_solution = BestAlgorithm::run(cost_matrix, points_cost, FromSolution(initial_random_solution));
    let best_cost = evaluate_solution(&best_solution, &cost_matrix, &points_cost);
    if verbose {
        println!("Best Cost: {}", best_cost);
    }
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    for _ in 0..1000 {
        let initial_solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, FromPoint(0));
        let solution = GreedyAlgorithm::run(cost_matrix, points_cost, FromSolution(initial_solution));
        let objective_value = evaluate_solution(&solution, cost_matrix, points_cost);
        let similarity = SimilarityMeasure::evaluate_similarity(&solution, &best_solution);
        pairs.push((objective_value, similarity));
    }
    let avg_similarity = pairs.iter().map(|(_, similarity)| similarity).sum::<i32>() / pairs.len() as i32;
    if verbose {
        println!("Comparing to Best Solution: Similarity measure: {}, Average Similarity: {}\n", SimilarityMeasure::get_name(), avg_similarity);
    }
    write_output_to_csv(output_path, pairs.clone(), best_cost, format!("{}_best", SimilarityMeasure::get_snaked_name()));

    pairs
}


pub fn check_similarity_avg<
    GreedyAlgorithm: TspAlgorithm,
    SimilarityMeasure: SimMeasure
>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, output_path: &Option<PathBuf>, verbose: bool) -> Vec<(i32, i32)> {


    let mut pairs: Vec<(i32, i32)> = Vec::new();


    let mut objective_values = Vec::new();
    let mut solutions = Vec::new();

    // map containing solution and tuple (objective value, average similarity to other sols)
    let mut sol_sim_map: HashMap<Vec<i32>, (i32, i32)> = HashMap::new();

    for _ in 0..1000 {
        let initial_solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, FromPoint(0));
        let solution = GreedyAlgorithm::run(cost_matrix, points_cost, FromSolution(initial_solution));
        let objective_value = evaluate_solution(&solution, cost_matrix, points_cost);
        solutions.push(solution);
        objective_values.push(objective_value);
    }

    for i in 0..1000 {
        let mut similarity = 0;
        for j in 0..1000 {
            if i != j {
                similarity += SimilarityMeasure::evaluate_similarity(&solutions[i], &solutions[j]);
            }
        }
        similarity /= 999;
        sol_sim_map.insert(solutions[i].clone(), (objective_values[i], similarity));
    }

    // get the pairs from the map
    for (_, pair) in sol_sim_map {
        pairs.push(pair);
    }

    let avg_similarity = pairs.iter().map(|(_, similarity)| similarity).sum::<i32>() / pairs.len() as i32;
    if verbose {
        println!("Comparing to all other solutions: Similarity measure: {}, Average Similarity: {}\n", SimilarityMeasure::get_name(), avg_similarity);
    }
    write_output_to_csv(output_path, pairs.clone(), 0, format!("{}_avg", SimilarityMeasure::get_snaked_name()));

    pairs
}

fn write_output_to_csv(output_path: &Option<PathBuf>, pairs: Vec<(i32, i32)>, best_objective_value: i32, similarity_name: String) {
    if let Some(path) = output_path {
        let output_path = path.join(format!("output_{}.csv", similarity_name));

        let mut wtr = csv::Writer::from_path(output_path).expect("Could not create output file");
        for (objective_value, similarity) in pairs {
            wtr.write_record(&[objective_value.to_string(), similarity.to_string()]).expect("Could not write record");
        }
        // write best objective value
        wtr.write_record(&["Best Objective Value".to_string(), best_objective_value.to_string()]).expect("Could not write record");
        wtr.flush().expect("Could not flush writer");
    }
}