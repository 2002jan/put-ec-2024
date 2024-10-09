use std::path::Path;
use run_utils::args::{Args, Command};
use tsp_algos::greedy_heuristics::random::RandomAlgorithm;
use tsp_algos::test_algorithm::test_tsp_algorithm;
use tsp_utils::coordinate_tsp_reader::load_from_coordinate_csv;

fn main() {
    let args = Args::build();

    let file_path = Path::new(&args.file);

    if !file_path.exists() {
        panic!("Such file does not exist")
    }

    let (cost_matrix, points_cost) = load_from_coordinate_csv(file_path);

    match args.command {
        Command::Task1 => {
            test_tsp_algorithm::<RandomAlgorithm>(&cost_matrix, &points_cost, true);
        }
    }

    println!("Thank you for running our amazing software solution written in RUST");
}
