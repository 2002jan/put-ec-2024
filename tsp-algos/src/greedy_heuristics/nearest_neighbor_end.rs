use tsp_utils::cost_matrix::CostMatrix;
use crate::{StartType, TspAlgorithm};

pub struct NearestNeighborEndAlgorithm {}

impl TspAlgorithm for NearestNeighborEndAlgorithm {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, start_from: StartType) -> Vec<i32> {
        let size = cost_matrix.size();
        let solution_size = ((size as f32) / 2.).ceil() as usize;

        let mut solution: Vec<usize> = start_from.get_starting_solution(solution_size);
        let mut visited = vec![false; size];

        for i in 0..solution.len() {
            visited[solution[i]] = true;
        }

        // Build the solution by adding the nearest node at the end of the path
        while solution.len() < solution_size as usize {
            // Get the last node in the current path
            let last_node = *solution.last().unwrap();

            // Find the nearest unvisited node
            let mut nearest_node = None;
            let mut nearest_cost = i32::MAX;

            for next_node in 0..cost_matrix.size() {
                if !visited[next_node as usize] {
                    let distance = cost_matrix.get(last_node as usize, next_node as usize) + points_cost[next_node as usize];

                    if distance < nearest_cost {
                        nearest_cost = distance;
                        nearest_node = Some(next_node);
                    }
                }
            }

            // Add the nearest node to the solution and mark it as visited
            if let Some(nearest) = nearest_node {
                solution.push(nearest);
                visited[nearest] = true;
            }
        }

        solution.iter().map(|x| *x as i32).collect()
    }

    fn name() -> String {
        String::from("Nearest Neighbor with adding the node at the end algorithm")
    }

    fn snaked_name() -> String {
        String::from("nearest_neighbor_end_algorithm")
    }
}