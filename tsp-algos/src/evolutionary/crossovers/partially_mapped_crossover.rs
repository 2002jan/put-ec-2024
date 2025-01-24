use std::collections::HashMap;
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::evolutionary::crossovers::Crossover;

pub struct PartiallyMappedCrossover {}

impl Crossover for PartiallyMappedCrossover {
    fn crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, _cost_matrix: &CostMatrix, _points_cost: &Vec<i32>) -> Vec<usize> {
        let solution_len = parent1.len();

        let mut rng = thread_rng();

        let point1 = rng.gen_range(0..solution_len - 1);
        let point2 = rng.gen_range(point1..solution_len);

        let mut new_solution = parent1.clone();

        let moved_1 = &parent1[point1..point2];
        let moved_2 = &parent2[point1..point2];

        new_solution[point1..point2].copy_from_slice(moved_2);

        let mut changes_1 = HashMap::new();

        for i in 0..moved_1.len() {
            changes_1.insert(moved_2[i], moved_1[i]);
        }

        for i in 0..solution_len {
            if i >= point1 && i < point2 {
                continue
            }

            if let Some(&new_val) = changes_1.get(&new_solution[i]) {
                let mut new_val = new_val;

                while moved_2.contains(&new_val) {
                    new_val = changes_1[&new_val]
                }

                new_solution[i] = new_val
            }
        }

        assert_eq!(new_solution.len(), new_solution.iter().cloned().collect::<std::collections::HashSet<_>>().len());

        new_solution
    }

    fn name() -> String {
        String::from("Partially mapped crossover")
    }

    fn snaked_name() -> String {
        String::from("partially_mapped_crossover")
    }
}