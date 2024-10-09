use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
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

    let output_path: Option<PathBuf> = match args.outputs_folder {
        None => None,
        Some(path) => {
            let current_datetime = format!("{}", chrono::Utc::now().format("%Y_%m_%d_%H_%M_%S"));
            let path = Path::new(&path).join(&current_datetime);

            if !path.exists() {
                create_dir_all(&path).expect("Could not create output folder");
            }

            Some(path)
        }
    };


    match args.command {
        Command::Task1 => {
            test_tsp_algorithm::<RandomAlgorithm>(&cost_matrix, &points_cost, &output_path, true);
        }
    }

    println!("Thank you for running our amazing software solution written in RUST");
}
