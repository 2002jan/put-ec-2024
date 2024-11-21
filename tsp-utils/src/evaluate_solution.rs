use crate::cost_matrix::CostMatrix;

pub fn evaluate_solution(solution: &Vec<i32>, cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> i32 {
    let solution_size = solution.len();

    let mut total_cost = 0;

    for i in 0..solution_size {
        let next_one = (i + 1) % solution_size;
        let current_point = solution[i] as usize;
        let next_point = solution[next_one] as usize;

        total_cost += points_cost[current_point] + cost_matrix.get(current_point, next_point);
    }

    total_cost
}

pub fn evaluate_solution_usize(solution: &Vec<usize>, cost_matrix: &CostMatrix, points_cost: &Vec<i32>) -> i32 {
    let solution_size = solution.len();

    let mut total_cost = 0;

    for i in 0..solution_size {
        let next_one = (i + 1) % solution_size;
        let current_point = solution[i];
        let next_point = solution[next_one];

        total_cost += points_cost[current_point] + cost_matrix.get(current_point, next_point);
    }

    total_cost
}
