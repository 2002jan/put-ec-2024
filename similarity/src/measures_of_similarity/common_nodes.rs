use crate::get_similarity::SimMeasure;
use std::collections::HashSet;
use std::hash::Hash;

pub struct CommonNodes {}
impl SimMeasure for CommonNodes {
    fn evaluate_similarity(solution1: &Vec<i32>, solution2: &Vec<i32>) -> i32 {
        let set1: HashSet<i32> = solution1.iter().map(|&x| x).collect();
        let set2: HashSet<i32> = solution2.iter().map(|&x| x).collect();
        let mut intersection = set1.intersection(&set2);
        intersection.count() as i32
    }

    fn get_name() -> String {
        String::from("Common Nodes")
    }

    fn get_snaked_name() -> String {
        String::from("common_nodes")
    }
}