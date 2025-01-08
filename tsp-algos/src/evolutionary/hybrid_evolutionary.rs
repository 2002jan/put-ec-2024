use std::marker::PhantomData;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution_usize;
use crate::{StartType, TspAlgorithm};
use crate::evolutionary::crossovers::Crossover;
use crate::evolutionary::mutations::Mutation;
use crate::local_search::neighbourhoods::two_edges_intra::TwoEdgesIntra;
use crate::local_search::search_types::LocalSearchType;
use crate::local_search::starting_solution::random_starting_solution::RandomStartingSolution;
use crate::local_search::starting_solution::StartingSolution;
use crate::StartType::FromStart;

const POPULATION_SIZE: usize = 20;
const MAX_RUN_TIME: u64 = 2;

struct HybridEvolutionaryChap {
    fitness: i32,
    solution: Vec<usize>,
}

pub struct HybridEvolutionary<
    T: LocalSearchType,
    M: Mutation,
    C: Crossover
> {
    t: PhantomData<T>,
    m: PhantomData<M>,
    c: PhantomData<C>,
}

impl<
    T: LocalSearchType,
    M: Mutation,
    C: Crossover
> HybridEvolutionary<T, M, C> {
    fn get_starting_population(cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<HybridEvolutionaryChap> {
        let mut population: Vec<HybridEvolutionaryChap> = Vec::new();

        for _ in 0..POPULATION_SIZE {
            let solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, FromStart);
            let solution = T::run::<TwoEdgesIntra>(cost_matrix, points_cost, solution);
            let fitness = evaluate_solution_usize(&solution, cost_matrix, points_cost);

            population.push(HybridEvolutionaryChap {
                fitness,
                solution,
            });
        }

        population
    }

    fn sort_population(population: &mut Vec<HybridEvolutionaryChap>) {
        population.sort_by_key(|chap| chap.fitness)
    }
}

impl<
    T: LocalSearchType,
    M: Mutation,
    C: Crossover
> TspAlgorithm for HybridEvolutionary<T, M, C> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, _start_from: StartType) -> Vec<i32> {
        let interval = Duration::from_secs(MAX_RUN_TIME);
        let start = Instant::now();

        let mut population = Self::get_starting_population(cost_matrix, points_cost);
        Self::sort_population(&mut population);

        let mut rng = thread_rng();

        while start.elapsed() < interval {
            let parent1 = rng.gen_range(0..POPULATION_SIZE);
            let mut parent2 = rng.gen_range(0..POPULATION_SIZE - 1);

            if parent2 >= parent1 {
                parent2 += 1;
            }

            let new_chap = C::crossover(
                &population[parent1].solution,
                &population[parent2].solution,
                cost_matrix,
                points_cost,
            );

            M::mutate(
                &new_chap,
                cost_matrix,
                points_cost,
            );

            let new_chap = T::run::<TwoEdgesIntra>(cost_matrix, points_cost, new_chap);

            let fitness = evaluate_solution_usize(
                &new_chap,
                cost_matrix,
                points_cost,
            );

            population.push(HybridEvolutionaryChap {
                fitness,
                solution: new_chap
            });

            Self::sort_population(&mut population);
            population.truncate(POPULATION_SIZE);
        }

        Self::sort_population(&mut population);
        population[0].solution.iter().map(|&x| x as i32).collect()
    }

    fn name() -> String {
        T::name() + " " + M::name().as_str() + " " + C::name().as_str() + " "+ "Hybrid Evolutionary algorithm"
    }

    fn snaked_name() -> String {
        T::snaked_name() + "_" + M::snaked_name().as_str() + "_" + C::snaked_name().as_str() + "_"+ "hybrid_evolutionary_algorithm"
    }
}