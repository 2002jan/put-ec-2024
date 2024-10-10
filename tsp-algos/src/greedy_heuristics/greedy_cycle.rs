use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;
use crate::TspAlgorithm;

pub struct GreedyCycle {}

impl TspAlgorithm for GreedyCycle {
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

            let mut new_min_cost = i32::MAX;
            let mut node_to_add = None;
            let mut best_insert_pos: usize = 0;

            for node in 0..size {
                if visited[node] {
                    continue;
                }

                for insert_pos in 0..current_solution_length + 1 {
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

                    if new_min_cost > new_cost {
                        new_min_cost = new_cost;
                        node_to_add = Some(node);
                        best_insert_pos = insert_pos
                    }
                }
            }

            if let Some(node_to_add) = node_to_add {
                solution.insert(best_insert_pos, node_to_add);
                current_cost = new_min_cost;
                visited[node_to_add] = true;
            }
        }

        solution.iter().map(|x| *x as i32).collect()
    }

    fn name() -> &'static str {
        "Greedy cycle algorithm"
    }

    fn snaked_name() -> &'static str {
        "greedy_cycle"
    }
}
