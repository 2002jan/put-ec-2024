use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution_usize;
use crate::{StartType, TspAlgorithm};

pub struct NearestNeighborAnyAlgorithm {}

impl TspAlgorithm for NearestNeighborAnyAlgorithm {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let size = cost_matrix.size();
        let solution_size = ((size as f32) / 2.).ceil() as usize;


        let mut solution: Vec<usize> = start_from.get_starting_solution(solution_size);
        let mut visited = vec![false; size];

        for i in 0..solution.len() {
            visited[solution[i]] = true;
        }

        let mut current_cost = evaluate_solution_usize(&solution, cost_matrix, points_cost);

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

                    if insert_pos == 0 {
                        new_cost += cost_matrix.get(solution[0], node)
                    } else if insert_pos == current_solution_length {
                        new_cost += cost_matrix.get(solution[current_solution_length - 1], node);
                    } else {
                        new_cost = new_cost
                            - cost_matrix.get(solution[insert_pos], solution[insert_pos - 1])
                            + cost_matrix.get(solution[insert_pos - 1], node)
                            + cost_matrix.get(solution[insert_pos], node);
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

    fn name() -> String {
        String::from("Nearest neighbor insert anywhere algorithm")
    }

    fn snaked_name() -> String {
        String::from("nearest_neighbor_any_algorithm")
    }
}