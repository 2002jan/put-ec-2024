use std::path::Path;
use csv::Writer;

pub fn write_output(solution: &Vec<i32>, output: &Path) {
    let mut writer = Writer::from_path(output).expect("Could not open output file");

    for point in solution {
        writer.write_record(&[point.into()]).expect("Could not write point");
    }

    writer.flush().expect("Could not flush output");
}