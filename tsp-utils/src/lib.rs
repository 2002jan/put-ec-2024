pub mod coordinate_tsp_reader;
pub mod cost_matrix;
pub mod evaluate_solution;
pub mod output_writer;

pub fn get_neighbouring_indexes(i: usize, solution_size: usize) -> (usize, usize) {
    let mut prev_node = if i == 0 { solution_size - 1 } else { i - 1 };
    let mut next_node = i + 1;

    if i == 0 {
        prev_node = solution_size - 1;
    }

    if next_node >= solution_size {
        next_node = 0;
    }

    (prev_node, next_node)
}