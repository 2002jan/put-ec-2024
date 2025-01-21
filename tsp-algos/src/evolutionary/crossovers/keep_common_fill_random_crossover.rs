use std::collections::HashSet;
use std::marker::PhantomData;
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use crate::evolutionary::crossovers::Crossover;
use crate::{StartType, TspAlgorithm};

pub struct KeepCommonFillRandomCrossover{
}

impl Crossover for KeepCommonFillRandomCrossover{
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

        let mut new_chap: Vec<usize> = parent1.clone()
            .iter()
            .filter(|&x| nodes_to_keep.contains(x))
            .map(|&x| x)
            .collect();


        let mut free_edges = Vec::with_capacity(points_cost.len() * size_of::<usize>());

        for point in 0..points_cost.len() {
            if !new_chap.contains(&point) {
                free_edges.push(point);
            }
        }

        let mut rng = thread_rng();

        while new_chap.len() < len {
            let node = free_edges.remove(rng.gen_range(0..free_edges.len()));

            new_chap.insert(rng.gen_range(0..new_chap.len()+1), node);
        }

        new_chap
    }

    fn name() -> String {
        "Keep Common Fill LS Crossover with ".to_owned()
    }

    fn snaked_name() -> String {
        "keep_common_fill_ls_crossover_with".to_owned()
    }
}