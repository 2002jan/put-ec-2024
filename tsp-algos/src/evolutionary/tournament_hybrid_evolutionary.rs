use std::marker::PhantomData;
use std::mem::size_of;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;
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

const POPULATION_SIZE: usize = 60;
const MAX_RUN_TIME: u64 = 2;

const CROSSOVER_CHANCE: f64 = 0.7;
const MUTATION_CHANCE: f64 = 0.7;

const TOURNAMENT_SIZE: usize = 3;

struct TournamentHybridEvolutionaryChap {
    fitness: i32,
    solution: Vec<usize>,
}

pub struct TournamentHybridEvolutionary<
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
> TournamentHybridEvolutionary<T, M, C> {
    fn get_starting_population(cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> Vec<TournamentHybridEvolutionaryChap> {
        let mut population: Vec<TournamentHybridEvolutionaryChap> = Vec::new();

        for _ in 0..POPULATION_SIZE {
            let solution = RandomStartingSolution::get_staring_solution(cost_matrix, points_cost, FromStart);
            let solution = T::run::<TwoEdgesIntra>(cost_matrix, points_cost, solution);
            let fitness = evaluate_solution_usize(&solution, cost_matrix, points_cost);

            population.push(TournamentHybridEvolutionaryChap {
                fitness,
                solution,
            });
        }

        population
    }

    fn sort_population(population: &mut Vec<TournamentHybridEvolutionaryChap>) {
        population.sort_by_key(|chap| chap.fitness)
    }
}

impl<
    T: LocalSearchType,
    M: Mutation,
    C: Crossover
> TspAlgorithm for TournamentHybridEvolutionary<T, M, C> {
    fn run(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, _start_from: StartType) -> Vec<i32> {
        let interval = Duration::from_secs(MAX_RUN_TIME);
        let start = Instant::now();

        let mut population = Self::get_starting_population(cost_matrix, points_cost);
        Self::sort_population(&mut population);

        let mut rng = thread_rng();

        while start.elapsed() < interval {
            let mut new_population: Vec<TournamentHybridEvolutionaryChap> = Vec::with_capacity(size_of::<TournamentHybridEvolutionaryChap>() * POPULATION_SIZE);

            while new_population.len() < POPULATION_SIZE {
                let parent1 = rng.gen_range(0..POPULATION_SIZE);

                let crossover = rng.gen_bool(CROSSOVER_CHANCE);
                let mutate = match crossover {
                    true => rng.gen_bool(MUTATION_CHANCE),
                    false => true
                };

                let new_chap: Vec<usize> = if crossover {
                    let mut parent2 = rng.gen_range(0..POPULATION_SIZE - 1);

                    if parent2 >= parent1 {
                        parent2 += 1;
                    }

                    C::crossover(
                        &population[parent1].solution,
                        &population[parent2].solution,
                        cost_matrix,
                        points_cost,
                    )
                } else {
                    population[parent1].solution.clone()
                };

                if mutate {
                    M::mutate(
                        &new_chap,
                        cost_matrix,
                        points_cost,
                    );
                }

                let new_chap = T::run::<TwoEdgesIntra>(cost_matrix, points_cost, new_chap);

                let fitness = evaluate_solution_usize(
                    &new_chap,
                    cost_matrix,
                    points_cost,
                );

                for i in 0..POPULATION_SIZE {
                    if fitness == population[i].fitness {
                        continue;
                    }
                }

                for i in 0..new_population.len() {
                    if fitness == new_population[i].fitness {
                        continue;
                    }
                }

                new_population.push(TournamentHybridEvolutionaryChap {
                    fitness,
                    solution: new_chap,
                })
            }

            while let Some(pop) = population.pop() {
                new_population.push(pop);
            }

            Self::sort_population(&mut new_population);

            //Elites are kept
            population.push(new_population.pop().unwrap());

            while population.len() < POPULATION_SIZE {
                let mut random_selector: Vec<usize> = (0..new_population.len()).collect();
                random_selector.shuffle(&mut rng);

                let tournament_participants: &[usize] = &random_selector[0..TOURNAMENT_SIZE];

                let min_index = tournament_participants.iter().min().unwrap();

                population.push(new_population.remove(*min_index));
            }
        }

        Self::sort_population(&mut population);

        population[0].solution.iter().map(|&x| x as i32).collect()
    }

    fn name() -> String {
        T::name() + " " + M::name().as_str() + " " + C::name().as_str() + " " + "Tournament hybrid Evolutionary algorithm"
    }

    fn snaked_name() -> String {
        T::snaked_name() + "_" + M::snaked_name().as_str() + "_" + C::snaked_name().as_str() + "_" + "tournament_hybrid_evolutionary_algorithm"
    }
}
