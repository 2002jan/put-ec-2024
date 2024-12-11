use std::collections::HashSet;
use crate::get_similarity::SimMeasure;

pub struct CommonEdges {}

impl SimMeasure for CommonEdges {
    fn evaluate_similarity(solution1: &Vec<i32>, solution2: &Vec<i32>) -> i32 {
        let mut common_edges = 0;
        let len = solution1.len();

        // Create a HashSet to store edges from solution1 (both normal and inverted)
        let edges1: HashSet<(i32, i32)> = solution1
            .iter()
            .enumerate()
            .map(|(i, &node)| (node, solution1[(i + 1) % len]))
            .flat_map(|(a, b)| vec![(a, b), (b, a)])
            .collect();

        // Iterate through solution2 edges and check if they exist in edges1
        for i in 0..len {
            let edge = (solution2[i], solution2[(i + 1) % len]);
            if edges1.contains(&edge) {
                common_edges += 1;
            }
        }

        common_edges
    }

    fn get_name() -> String {
        String::from("Common Edges")
    }

    fn get_snaked_name() -> String {
        String::from("common_edges")
    }
}