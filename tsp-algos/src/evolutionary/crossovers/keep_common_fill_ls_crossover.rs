use std::collections::HashSet;
use std::marker::PhantomData;
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::evolutionary::crossovers::Crossover;
use crate::{StartType, TspAlgorithm};

pub struct KeepCommonFillLSCrossover<
F: TspAlgorithm
> {
    f: PhantomData<F>
}

impl<
F: TspAlgorithm
>  Crossover for KeepCommonFillLSCrossover<F> {
    fn crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<usize> {
        let len = parent1.len();

        let edges1: HashSet<(usize, usize)> = parent1
            .iter()
            .enumerate()
            .map(|(i, &node)| (node, parent1[(i + 1) % len]))
            .flat_map(|(a, b)| vec![(a, b), (b, a)])
            .collect();

        let mut nodes_to_keep: HashSet<usize> = HashSet::new();

        for i in 0..len {
            let edge = (parent1[i], parent2[(i + 1) % len]);
            if edges1.contains(&edge) {
                nodes_to_keep.insert(edge.0);
                nodes_to_keep.insert(edge.1);
            }
        }

        let new_chap: Vec<usize> = parent1.clone()
            .iter()
            .filter(|&x| nodes_to_keep.contains(x))
            .map(|&x| x)
            .collect();

        let start_from = if new_chap.len() == 0 {
            StartType::FromPoint(thread_rng().gen_range(0..points_cost.len()))
        } else {
            StartType::FromSolution(new_chap)
        };

        F::run(
            cost_matrix,
            points_cost,
            start_from
        ).iter().map(|&x| x as usize).collect()
    }

    fn name() -> String {
        "Keep Common Fill LS Crossover with ".to_owned() + F::name().as_str()
    }

    fn snaked_name() -> String {
        "keep_common_fill_ls_crossover_with".to_owned() + F::snaked_name().as_str()
    }
}