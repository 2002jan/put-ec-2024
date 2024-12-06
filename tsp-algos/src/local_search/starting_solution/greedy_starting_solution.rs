use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution;
use crate::greedy_heuristics::greedy_cycle::GreedyCycle;
use crate::greedy_heuristics::greedy_weighted_2regret_cycle::GreedyWeighted2Regret;
use crate::greedy_heuristics::nearest_neighbor_any::NearestNeighborAnyAlgorithm;
use crate::local_search::starting_solution::StartingSolution;
use crate::{StartType, TspAlgorithm};
use crate::StartType::FromPoint;

pub struct GreedyStartingSolution;

impl StartingSolution for GreedyStartingSolution {
    fn get_staring_solution(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<usize> {

        let start_node;

        if let StartType::FromPoint(point) = start_from {
            start_node = point;
        } else {
            let size = cost_matrix.size();

            let mut rng = thread_rng();
            start_node = rng.gen_range(0..size)
        }

        let greedy_cycle_solution = GreedyCycle::run(cost_matrix, points_cost, FromPoint(start_node));
        let weighted_2regret_solution = GreedyWeighted2Regret::run(cost_matrix, points_cost, FromPoint(start_node));
        let nearest_neighbor_solution = NearestNeighborAnyAlgorithm::run(cost_matrix, points_cost, FromPoint(start_node));

        let greedy_cycle_score = evaluate_solution(&greedy_cycle_solution, cost_matrix, points_cost);
        let weighted_2regret_score = evaluate_solution(&weighted_2regret_solution, cost_matrix, points_cost);
        let nearest_neighbor_score = evaluate_solution(&nearest_neighbor_solution, cost_matrix, points_cost);

        let best_solution = if greedy_cycle_score <= weighted_2regret_score && greedy_cycle_score <= nearest_neighbor_score {
            greedy_cycle_solution
        } else if weighted_2regret_score <= nearest_neighbor_score {
            weighted_2regret_solution
        } else {
            nearest_neighbor_solution
        };

        best_solution.iter().map(|&x| x as usize).collect::<Vec<usize>>()
    }

    fn name() -> String {
        String::from("Greedy Start")
    }

    fn snaked_name() -> String {
        String::from("greedy_start")
    }
}
