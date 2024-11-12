use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, LinkedList};
use std::mem::size_of;
use tsp_utils::cost_matrix::CostMatrix;
use tsp_utils::evaluate_solution::evaluate_solution;
use tsp_utils::get_neighbouring_indexes;
use crate::local_search::neighbourhoods::{LocalSearchMove, LocalSearchNeighbourhood};
use crate::local_search::neighbourhoods::LocalSearchMove::{Inter, Intra};
use crate::local_search::search_types::LocalSearchType;
use crate::local_search::search_types::steepest_deltas::ImprovingMove::{ImprovingInterMove, ImprovingIntraMove};
use crate::local_search::search_types::steepest_deltas::MoveValidity::{Remove, Reverse, Skip, Valid};

type NodePoses = HashMap<usize, (bool, usize)>;

#[derive(Eq)]
struct ImprovingMoveWrapper {
    delta: i32,
    mov: ImprovingMove,
}

enum MoveValidity {
    Valid,
    Reverse,
    Skip,
    Remove,
}

impl ImprovingMoveWrapper {
    fn check_move_validity(&self, current_solution: &Vec<usize>, nodes_poses: &NodePoses) -> MoveValidity {
        match self.mov {
            ImprovingIntraMove(start, start_prev, target, target_next) =>
                Self::check_intra_move_validity(start, start_prev, target, target_next, current_solution, nodes_poses),
            ImprovingInterMove(start, start_prev, start_next, target) =>
                Self::check_inter_move_validity(start, start_prev, start_next, target, current_solution, nodes_poses)
        }
    }

    fn check_intra_move_validity(start: usize, start_prev: usize, target: usize, target_next: usize, current_solution: &Vec<usize>, nodes_poses: &NodePoses) -> MoveValidity {
        let start_pos = nodes_poses[&start];
        let target_pos = nodes_poses[&target];
        let solution_size = current_solution.len();

        if !start_pos.0 || !target_pos.0 {
            return Remove;
        }

        let (start_pos_prev, start_pos_next) = get_neighbouring_indexes(start_pos.1, solution_size);
        let (target_pos_prev, target_pos_next) = get_neighbouring_indexes(target_pos.1, solution_size);

        if current_solution[start_pos_prev] == start_prev && current_solution[target_pos_next] == target_next {
            return Valid;
        }

        let (start_rev, target_rev) = (current_solution[start_pos_next] == start_prev, current_solution[target_pos_prev] == target_next);

        if start_rev && target_rev {
            Reverse
        } else if start_rev || target_rev {
            Skip
        } else {
            Remove
        }
    }

    fn check_inter_move_validity(start: usize, start_prev: usize, start_next: usize, target: usize, current_solution: &Vec<usize>, nodes_poses: &NodePoses) -> MoveValidity {
        let start_pos = nodes_poses[&start];
        let target_pos = nodes_poses[&target];
        let solution_size = current_solution.len();

        if !start_pos.0 || target_pos.0 {
            return Remove;
        }

        let (start_pos_prev, start_pos_next) = get_neighbouring_indexes(start_pos.1, solution_size);
        let (solution_prev, solution_next) = (current_solution[start_pos_prev], current_solution[start_pos_next]);

        if (solution_prev == start_prev && solution_next == start_next)
            || (solution_prev == start_next && solution_next == start_prev) {
            return Valid;
        }

        Remove
    }

    fn update_nodes_poses(&self, current_solution: &Vec<usize>, nodes_poses: &mut NodePoses) {
        match self.mov {
            ImprovingIntraMove(_, _, _, _) => Self::update_nodes_poses_intra(current_solution, nodes_poses),
            ImprovingInterMove(start, _, _, target) => Self::update_nodes_poses_inter(nodes_poses, start, target)
        }
    }

    fn update_nodes_poses_inter(nodes_poses: &mut NodePoses, start: usize, target: usize) {
        let start_pos = nodes_poses[&start];
        let target_pos = nodes_poses[&target];

        nodes_poses.insert(start, target_pos);
        nodes_poses.insert(target, start_pos);
    }

    fn update_nodes_poses_intra(current_solution: &Vec<usize>, node_poses: &mut NodePoses) {
        for i in 0..current_solution.len() {
            node_poses.insert(current_solution[i], (true, i));
        }
    }
}

#[derive(Eq, PartialEq)]
enum ImprovingMove {
    // start, start_prev, target, target_next
    ImprovingIntraMove(usize, usize, usize, usize),
    // start, start_prev, start_next, target
    ImprovingInterMove(usize, usize, usize, usize),
}

pub struct SteepestDeltasLocalSearch {
    improving_moves: BinaryHeap<ImprovingMoveWrapper>,
}

impl SteepestDeltasLocalSearch {
    fn fill_improving_moves<N: LocalSearchNeighbourhood>(&mut self, cost_matrix: &CostMatrix, points_cost: &Vec<i32>, current_solution: &Vec<usize>, free_nodes: &Vec<usize>) {
        let solution_size = current_solution.len();
        let free_nodes_size = free_nodes.len();

        for start in 0..solution_size {
            let (start_prev, start_next) = get_neighbouring_indexes(start, solution_size);

            for target in 0..solution_size {
                if start == target {
                    continue;
                }

                let (_, target_next) = get_neighbouring_indexes(target, solution_size);
                let mov = Intra(start, target);
                let change = N::evaluate_move(cost_matrix, points_cost, &mov, current_solution, free_nodes);

                if change < 0 {
                    self.improving_moves.push(ImprovingMoveWrapper {
                        delta: change,
                        mov: ImprovingIntraMove(
                            current_solution[start],
                            current_solution[start_prev],
                            current_solution[target],
                            current_solution[target_next],
                        ),
                    })
                }
            }

            for target in 0..free_nodes_size {
                let mov = Inter(start, target);
                let change = N::evaluate_move(cost_matrix, points_cost, &mov, current_solution, free_nodes);

                if change < 0 {
                    self.improving_moves.push(ImprovingMoveWrapper {
                        delta: change,
                        mov: ImprovingInterMove(
                            current_solution[start],
                            current_solution[start_prev],
                            current_solution[start_next],
                            free_nodes[target],
                        ),
                    })
                }
            }
        }
    }
}

impl LocalSearchType for SteepestDeltasLocalSearch {
    fn new(_solution_size: usize, _free_nodes_size: usize) -> Self {
        SteepestDeltasLocalSearch {
            improving_moves: BinaryHeap::new(),
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

        let mut neighbourhood = Self::new(solution_size, free_nodes_size);
        neighbourhood.fill_improving_moves::<N>(cost_matrix, points_cost, &current_solution, &free_nodes);

        // Five head move
        current_solution.reverse();
        neighbourhood.fill_improving_moves::<N>(cost_matrix, points_cost, &current_solution, &free_nodes);
        current_solution.reverse();

        let mut nodes_poses: NodePoses = HashMap::new();

        for (i, node) in current_solution.iter().enumerate() {
            nodes_poses.insert(*node, (true, i));
        }

        for (i, node) in free_nodes.iter().enumerate() {
            nodes_poses.insert(*node, (false, i));
        }

        let mut moves_to_re_add: LinkedList<ImprovingMoveWrapper> = LinkedList::new();

        while let Some(mov) = neighbourhood.improving_moves.pop() {
            let call = mov.check_move_validity(&current_solution, &nodes_poses);

            let next_move = match call {
                Valid => {
                    match mov.mov {
                        ImprovingIntraMove(start, _, target, _) => {
                            Intra(nodes_poses[&start].1, nodes_poses[&target].1)
                        }
                        ImprovingInterMove(start, _, _, target) => {
                            Inter(nodes_poses[&start].1, nodes_poses[&target].1)
                        }
                    }
                }
                Reverse => {
                    match mov.mov {
                        ImprovingIntraMove(start, _, target, _) => {
                            Intra(nodes_poses[&target].1, nodes_poses[&start].1)
                        }
                        ImprovingInterMove(_, _, _, _) => panic!("EEEEEEE What am I supposed to do?")
                    }
                }
                Skip => {
                    moves_to_re_add.push_back(mov);
                    continue;
                }
                Remove => {
                    continue;
                }
            };

            N::apply_move(&next_move, &mut current_solution, &mut free_nodes);

            mov.update_nodes_poses(&current_solution, &mut nodes_poses);

            let mut new_moves = N::get_new_moves(&next_move, solution_size, free_nodes_size);

            while let Some(new_move) = new_moves.pop_back() {
                let change = N::evaluate_move(cost_matrix, points_cost, &new_move, &current_solution, &free_nodes);

                if change < 0 {
                    neighbourhood.improving_moves.push(ImprovingMoveWrapper {
                        delta: change,
                        mov: match new_move {
                            Intra(start, target) => {
                                let (start_prev, _) = get_neighbouring_indexes(start, solution_size);
                                let (_, target_next) = get_neighbouring_indexes(target, solution_size);

                                ImprovingIntraMove(
                                    current_solution[start],
                                    current_solution[start_prev],
                                    current_solution[target],
                                    current_solution[target_next],
                                )
                            }
                            Inter(start, target) => {
                                let (start_prev, start_next) = get_neighbouring_indexes(start, solution_size);

                                ImprovingInterMove(
                                    current_solution[start],
                                    current_solution[start_prev],
                                    current_solution[start_next],
                                    free_nodes[target],
                                )
                            }
                        },
                    })
                }
            }

            while let Some(re_add_move) = moves_to_re_add.pop_back() {
                neighbourhood.improving_moves.push(re_add_move);
            }
        }

        current_solution
    }

    fn next(&mut self) -> Option<LocalSearchMove> {
        todo!()
    }

    fn name() -> String {
        String::from("Steepest Deltas")
    }

    fn snaked_name() -> String {
        String::from("steepest_deltas")
    }
}

impl Ord for ImprovingMoveWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.delta.cmp(&self.delta)
    }
}

impl PartialOrd for ImprovingMoveWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for ImprovingMoveWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.delta == other.delta
    }
}
