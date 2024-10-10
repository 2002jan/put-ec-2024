# Repository for Evolutionary Computation tasks 

## Authors 
- Michał Kamiński 
- Jan Indrzejczak 

## Running the experiments
```bash
cargo run <command> <problem-file-path> <outputs-file-path>
```
where `<command>` is one of the following:
- `task1` - runs the first task of the assignment (TSP problem)

## Visualization usage 
Python script `visualize.py` is used to visualize the results of the algorithm. It is present in the visualization directory. To use it type the following command 
```bash
python visualize.py <path_to_file> <path_to_problem_instance>
```
where `<path_to_file>` is the path to the file with the results of the algorithm, represented as a csv file with node indices, and `<path_to_problem_instance>` is the path to the file with the problem instance (defaults to data/TSPA.csv).

You can also use a shell script `run_visualization.sh` to run the visualization script. It will iterate through the outputs directory and its subdirectories and visualize data from any csv files found there.
```bash
.\run_visualization.sh 
```

