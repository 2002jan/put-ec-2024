use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::get_neighbouring_indexes;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};
use crate::local_search::search_types::LocalSearchType;

const CLOSEST_CANDIDATES: usize = 10;

#[derive(Eq)]
struct CandidateNode {
    node: usize,
    distance: i32,
}

impl Ord for CandidateNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for CandidateNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for CandidateNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

pub struct SteepestCandidateLocalSearch {
    current_start: usize,
    next_start: usize,
    next_closest_node: usize,
    nodes_in_solution: HashSet<usize>,
    current_start_next_index: Option<usize>,
    intra_size: usize
}

impl SteepestCandidateLocalSearch {
    fn next_candidate(&mut self, current_solution: &Vec<usize>, free_nodes: &Vec<usize>, closest_nodes: &HashMap<usize, [usize; CLOSEST_CANDIDATES]>) -> Option<LocalSearchMove> {
        if self.next_closest_node >= CLOSEST_CANDIDATES || self.next_start == 0 {
            if self.next_start == 0 {
                for item in current_solution {
                    self.nodes_in_solution.insert(*item);
                }
            };

            self.current_start = self.next_start;
            self.next_start += 1;
            self.next_closest_node = 0;

            if self.current_start >= current_solution.len() {
                return None;
            }
        }

        let candidate = closest_nodes[&self.current_start][self.next_closest_node];

        let start;

        if self.nodes_in_solution.contains(&candidate){
            start = self.current_start;
            self.next_closest_node += 1;
        } else {
            start = match self.current_start_next_index {
                None => {
                    let (prev, next) = get_neighbouring_indexes(self.current_start, self.intra_size);
                    self.current_start_next_index = Some(next);

                    prev
                }
                Some(next) => {
                    self.next_closest_node += 1;

                    next
                }
            };
        }



        if self.nodes_in_solution.contains(&candidate) {
            let target_pos = current_solution.iter().position(|&x| x == candidate).unwrap();

            if target_pos > start {
                Some(Intra(start, target_pos))
            } else {
                Some(Intra(target_pos, start))
            }
        } else {
            Some(Inter(start, free_nodes.iter().position(|&x| x == candidate).unwrap()))
        }
    }
}

impl LocalSearchType for SteepestCandidateLocalSearch {
    fn new(solution_size: usize, _free_nodes_size: usize) -> Self {
        let nodes_in_solution_set = HashSet::new();

        SteepestCandidateLocalSearch {
            current_start: 0,
            next_start: 0,
            next_closest_node: 0,
            nodes_in_solution: nodes_in_solution_set,
            current_start_next_index: None,
            intra_size: solution_size
        }
    }

    fn run<N: LocalSearchNeighbourhood>(cost_matrix: &CostMatrix, points_cost: &Vec<i32>, starting_solution: Vec<usize>) -> Vec<usize> {
        let mut current_solution = starting_solution;
        let mut free_nodes = Vec::with_capacity((points_cost.len() - current_solution.len()) * size_of::<usize>());

        for i in 0..points_cost.len() {
            if !current_solution.contains(&i) {
                free_nodes.push(i);
            }
        }

        let solution_size = current_solution.len();
        let free_nodes_size = free_nodes.len();
        let mut closest_nodes: HashMap<usize, [usize; CLOSEST_CANDIDATES]> = HashMap::new();

        for node in 0..cost_matrix.size() {
            let mut candidates_heap = BinaryHeap::new();

            for neighbour in 0..cost_matrix.size() {
                if neighbour == node {
                    continue;
                }

                candidates_heap.push(CandidateNode {
                    distance: cost_matrix.get(node, neighbour) + points_cost[neighbour],
                    node: neighbour
                });
            }

            let mut closest_nodes_list = [0; CLOSEST_CANDIDATES];

            for i in 0..CLOSEST_CANDIDATES {
                closest_nodes_list[i] = candidates_heap.pop().unwrap().node;
            }

            closest_nodes.insert(node, closest_nodes_list);
        }

        loop {
            let mut neighbourhood_iterator = Self::new(solution_size, free_nodes_size);

            let mut best_change = 0;
            let mut bets_move: Option<LocalSearchMove> = None;

            loop {
                let next_move = neighbourhood_iterator.next_candidate(&current_solution, &free_nodes, &closest_nodes);

                let next_move = match next_move {
                    None => {
                        break;
                    }
                    Some(mov) => mov
                };

                let change = N::evaluate_move(cost_matrix, points_cost, &next_move, &current_solution, &free_nodes);

                if change < best_change {
                    best_change = change;
                    bets_move = Some(next_move);
                }
            }

            if let Some(mov) = bets_move {
                N::apply_move(&mov, &mut current_solution, &mut free_nodes);
            } else {
                break;
            }
        }

        current_solution
    }

    fn next(&mut self) -> Option<LocalSearchMove> {
        None
    }

    fn name() -> String {
        String::from("Steepest Candidate")
    }

    fn snaked_name() -> String {
        String::from("steepest_candidate")
    }
}
