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
python .\visualization\visualize.py  <path_to_file> <path_to_problem_instance> <path_for_plot_outputs>
```
where `<path_to_file>` is the path to the file with the results of the algorithm, represented as a csv file with node indices, and `<path_to_problem_instance>` is the path to the file with the problem instance (defaults to data/TSPA.csv). 
The `<path_for_plot_outputs>` specifies where the plots will be saved. If not provided, it creates plots directory. 

You can also use a shell script `run_visualization.sh` to run the visualization script. Provide the path to the directory with the results and the path to the dataset. 
```bash
.\visualization\run_visualization.ps1 <path_to_results> <path_to_problem_instance>
```
The arguments are optional. If not provided, the script will use the default values `<path_to_results>` is outputs directory and <path_to_problem_instance> is data/TSPA.csv.
