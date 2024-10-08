use std::path::Path;
use csv::StringRecord;
use crate::CostMatrix;

struct CoordinatePoint {
    x: i32,
    y: i32,
    cost: i32,
}

pub fn load_from_coordinate_csv(path: &Path) -> (CostMatrix, Vec<i32>) {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_path(path)
        .expect("Invalid csv path");

    let records = reader.records();
    let mut points: Vec<CoordinatePoint> = vec![];

    for result in records {
        let record = result.unwrap();
        let size = record.len();

        if size < 3 {
            panic!("Invalid csv row length")
        }

        points.push(CoordinatePoint {
            x: get_from_string_record(&record, 0),
            y: get_from_string_record(&record, 1),
            cost: get_from_string_record(&record, 2),
        })
    }

    let points_count = points.len();
    let mut cost_matrix = CostMatrix::new(points_count);
    let mut points_costs: Vec<i32> = vec![];

    for i in 0..points_count {
        let point1 = &points[i];
        points_costs.push(point1.cost);

        for j in i+1..points_count {
            let point2 = &points[j];
            let distance = get_euclidean_distance(point1.x, point1.y, point2.x, point2.y);

            cost_matrix.set(i, j, distance);
            cost_matrix.set(j, i, distance);
        }
    }

    (cost_matrix, points_costs)
}

fn get_from_string_record(record: &StringRecord, i: usize) -> i32 {
    record.get(i).expect("Invalid csv row index").parse().expect("Could not parse string into int")
}

fn get_euclidean_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32{
    let distance = f32::sqrt(((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f32);

    distance.round() as i32
}