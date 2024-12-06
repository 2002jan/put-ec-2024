use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use run_utils::args::{Args, Command};
use tsp_algos::greedy_heuristics::greedy_cycle::GreedyCycle;
use tsp_algos::greedy_heuristics::nearest_neighbor_any::NearestNeighborAnyAlgorithm;
use tsp_algos::greedy_heuristics::random::RandomAlgorithm;
use tsp_algos::greedy_heuristics::nearest_neighbor_end::NearestNeighborEndAlgorithm;
use tsp_algos::greedy_heuristics::greedy_2regret_cycle::Greedy2Regret;
use tsp_algos::greedy_heuristics::greedy_weighted_2regret_cycle::GreedyWeighted2Regret;
use tsp_algos::local_search::iterated_local_search::IteratedLocalSearch;
use tsp_algos::local_search::starting_solution::greedy_starting_solution::GreedyStartingSolution;
use tsp_algos::local_search::local_search::LocalSearch;
use tsp_algos::local_search::multiple_start_local_search::MultipleStartLocalSearch;
use tsp_algos::local_search::neighbourhoods::two_edges_intra::TwoEdgesIntra;
use tsp_algos::local_search::neighbourhoods::two_nodes_intra::TwoNodesIntra;
use tsp_algos::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use tsp_algos::local_search::search_types::greedy::GreedyLocalSearch;
use tsp_algos::local_search::search_types::steepest::SteepestLocalSearch;
use tsp_algos::local_search::search_types::steepest_candidate::SteepestCandidateLocalSearch;
use tsp_algos::local_search::search_types::steepest_deltas::SteepestDeltasLocalSearch;
use tsp_algos::other::destroy_methods::random_destroy::RandomDestroy;
use tsp_algos::other::large_neighborhood_search::{LargeNeighborhoodSearch, LargeNeighborhoodSearchWith};
use tsp_algos::test_algorithm::{test_tsp_algorithm, test_tsp_algorithm_with_runs};
use tsp_utils::coordinate_tsp_reader::load_from_coordinate_csv;

fn main() {
    let args = Args::build();

    let file_path = Path::new(&args.file);

    if !file_path.exists() {
        panic!("Such file does not exist")
    }

    let (cost_matrix, points_cost) = load_from_coordinate_csv(file_path);

    let output_path: Option<PathBuf> = match args.outputs_folder {
        None => None,
        Some(path) => {
            let current_datetime = format!("{}", chrono::Utc::now().format("%Y_%m_%d_%H_%M_%S"));
            let path = Path::new(&path).join(&current_datetime);

            if !path.exists() {
                create_dir_all(&path).expect("Could not create output folder");
            }

            println!("Results will be saved to {current_datetime}\n");

            Some(path)
        }
    };


    match args.command {
        Command::Task1 => {
            test_tsp_algorithm::<RandomAlgorithm>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<NearestNeighborEndAlgorithm>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<NearestNeighborAnyAlgorithm>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<GreedyCycle>(&cost_matrix, &points_cost, &output_path, true);
        }
        Command::Task2 => {
            test_tsp_algorithm::<NearestNeighborAnyAlgorithm>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<GreedyCycle>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<Greedy2Regret>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<GreedyWeighted2Regret>(&cost_matrix, &points_cost, &output_path, true);
        }
        Command::Task3 => {
            test_tsp_algorithm::<LocalSearch<GreedyLocalSearch, TwoNodesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<GreedyLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<GreedyLocalSearch, TwoNodesIntra, GreedyStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<GreedyLocalSearch, TwoEdgesIntra, GreedyStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);

            test_tsp_algorithm::<LocalSearch<SteepestLocalSearch, TwoNodesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<SteepestLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<SteepestLocalSearch, TwoNodesIntra, GreedyStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<SteepestLocalSearch, TwoEdgesIntra, GreedyStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
        }
        Command::Task4 => {
            test_tsp_algorithm::<LocalSearch<SteepestCandidateLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<SteepestLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
        }
        Command::Task5 => {
            test_tsp_algorithm::<LocalSearch<SteepestDeltasLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
            test_tsp_algorithm::<LocalSearch<SteepestLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true);
        }
        Command::Task6 => {
            test_tsp_algorithm_with_runs::<MultipleStartLocalSearch<SteepestLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true, 20);
            test_tsp_algorithm_with_runs::<MultipleStartLocalSearch<SteepestDeltasLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true, 20);
            test_tsp_algorithm_with_runs::<IteratedLocalSearch<SteepestDeltasLocalSearch, TwoEdgesIntra, RandomStartingSolution>>(&cost_matrix, &points_cost, &output_path, true, 20);
        }
        Command::Task7 => {
            test_tsp_algorithm_with_runs::<LargeNeighborhoodSearch<GreedyWeighted2Regret, RandomDestroy>>(&cost_matrix, &points_cost, &output_path, true, 20);
            test_tsp_algorithm_with_runs::<LargeNeighborhoodSearchWith<SteepestDeltasLocalSearch, GreedyWeighted2Regret, RandomDestroy>>(&cost_matrix, &points_cost, &output_path, true, 20);
        }
    }

    println!("Thank you for running our amazing software solution written in RUST");
}
