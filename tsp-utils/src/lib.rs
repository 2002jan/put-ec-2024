pub mod coordinate_tsp_reader;

pub struct CostMatrix {
    data: Vec<i32>,
    size: usize,
}

impl CostMatrix {
    fn new(size: usize) -> CostMatrix {
        let vector_length = size.pow(2);

        CostMatrix {
            data: vec![0; vector_length],
            size,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize{
        (self.size * y) + x
    }

    fn get(&self, x: usize, y: usize) -> i32{
        self.data[self.get_index(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, cost: i32) {
        let index = self.get_index(x, y);

        self.data[index] = cost;
    }
}