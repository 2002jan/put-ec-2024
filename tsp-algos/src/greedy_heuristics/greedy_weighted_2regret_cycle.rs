use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;
use crate::TspAlgorithm;

const REGRET_WEIGHT: f32 = 0.5;

pub struct GreedyWeighted2Regret {}

impl TspAlgorithm for GreedyWeighted2Regret {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: Option<i32>) -> Vec<i32> {
        let size = cost_matrix.size();
        let solution_size = ((size as f32) / 2.).ceil() as usize;

        let mut solution: Vec<usize> = Vec::with_capacity(solution_size * size_of::<i32>());
        let mut visited = vec![false; size];

        let start_node = start_from.unwrap_or(0) as usize;
        visited[start_node] = true;
        solution.push(start_node);

        let mut current_cost = points_cost[start_node];

        while solution.len() < solution_size {
            let current_solution_length = solution.len();

            let mut max_score = f32::MIN;
            let mut best_node = None;
            let mut best_insert_pos: usize = 0;
            let mut new_score = 0;

            for node in 0..size {
                if visited[node] {
                    continue;
                }

                let mut best_cost = i32::MAX;
                let mut second_best_cost = i32::MAX;
                let mut node_best_insert_pos: usize = 0;

                for insert_pos in 0..=current_solution_length {
                    let mut new_cost = current_cost + points_cost[node];

                    if insert_pos == 0 || insert_pos == current_solution_length {
                        new_cost = new_cost
                            - cost_matrix.get(solution[0], solution[current_solution_length - 1])
                            + cost_matrix.get(solution[0], node)
                            + cost_matrix.get(node, solution[current_solution_length - 1]);
                    } else {
                        new_cost = new_cost
                            - cost_matrix.get(solution[insert_pos], solution[insert_pos - 1])
                            + cost_matrix.get(solution[insert_pos], node)
                            + cost_matrix.get(solution[insert_pos - 1], node);
                    }

                    // Update the best and second-best costs
                    if new_cost < best_cost {
                        second_best_cost = best_cost;
                        best_cost = new_cost;
                        node_best_insert_pos = insert_pos;
                    } else if new_cost < second_best_cost {
                        second_best_cost = new_cost;
                    }
                }

                let regret = second_best_cost - best_cost;

                let score = (regret as f32 * REGRET_WEIGHT ) - ((best_cost - current_cost) as f32 * (1.0 - REGRET_WEIGHT));

                if score > max_score {
                    max_score = score;
                    best_node = Some(node);
                    best_insert_pos = node_best_insert_pos;
                    new_score = best_cost;
                }
            }

            if let Some(node_to_add) = best_node {
                solution.insert(best_insert_pos, node_to_add);
                current_cost = new_score;
                visited[node_to_add] = true;
            }
        }

        solution.iter().map(|x| *x as i32).collect()
    }

    fn name() -> &'static str {
        "Greedy Regret Heuristic with weighted 2-Regret"
    }

    fn snaked_name() -> &'static str {
        "greedy_weighted_2regret"
    }
}
