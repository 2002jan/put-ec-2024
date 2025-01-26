import os.path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.figure import Figure

"""
RM -> Random replace mutation
MM -> Random move mutation
FC -> Keep Common Fill Ls Crossover
PMX -> Partially mapped crossover
HE -> Hybrid Evo
TE -> Tournaments hybrid evo
"""
names_translations = {
    "Random Start Two Edges Intra Steepest Deltas Iterated Local Search": "ILS",
    "Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Hybrid Evolutionary algorithm": "HE_RM_FC",
    "Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Tournament hybrid Evolutionary algorithm": "TE_RM_FC",
    "Steepest Deltas Random Replace Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm": "TE_RM_PMX",
    "Steepest Deltas Random Move Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm": "TE_MM_PMX",
    "First Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search": "HE_RM_FC_ILS",
    "First Steepest Deltas Random Move Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search": "TE_MM_FC_ILS",
    "First Steepest Deltas Random Replace Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search": "TE_RM_PMX_ILS",
    "First Steepest Deltas Random Move Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search": "TE_MM_PMX_ILS",
}


def generate_graphs(file_path: str, outputs_folder: str, output_suffix: str, problem_name: str):
    if not os.path.exists(outputs_folder):
        os.mkdir(outputs_folder)

    all_results = []

    current = None

    with open(file_path, "r") as file:
        for line in file:
            if current is None and line.startswith("Results for "):
                full_name = line.replace('Results for ', '').replace('\n', '')
                name = names_translations[full_name] if full_name in names_translations else '_'.join(
                    full_name.lower().split(" "))
                current = {'name': name}
                continue

            if current is None:
                continue

            if line.startswith("Min cost: "):
                current['min'] = int(line.replace('Min cost: ', ''))
            elif line.startswith("Max cost: "):
                current['max'] = int(line.replace('Max cost: ', ''))
            elif line.startswith("Average cost: "):
                current['avg'] = int(line.replace('Average cost: ', ''))
                all_results.append(current)
                current = None

    all_results.sort(key=lambda x: x['min'])

    names = [x['name'] for x in all_results]
    avg = [x['avg'] for x in all_results]
    mins_dff = [x['avg'] - x['min'] for x in all_results]
    maxes_diff = [x['max'] - x['avg'] for x in all_results]
    mins = [x['min'] for x in all_results]
    maxes = [x['max'] for x in all_results]

    min_min = min(mins)
    min_max = min(maxes)

    yerr = [mins_dff, maxes_diff]

    fig, ax = plt.subplots(1, 1, figsize=(16, 9))

    # axs.scatter(names, avg)
    ax.errorbar(names, avg, yerr=yerr, capsize=7, fmt="r--o", ecolor="black", elinewidth=3, capthick=2)
    ax.plot(names, [min_min for x in names], linestyle='--', alpha=0.5)
    ax.plot(names, [min_max for x in names], linestyle='--', alpha=0.5)
    ax.set_ylabel("Scores")
    ax.set_xlabel("Algorithm")
    ax.set_title(f"Solution ranges for given method [{problem_name}]")

    fig.savefig(os.path.join(outputs_folder, f"ranges_{output_suffix}.png"))

    fig, ax = plt.subplots(1, 1, figsize=(16, 9))

    p = ax.bar(names, mins)
    ax.bar_label(p)
    ax.set_ylim([min(mins) - 100, max(mins) + 100])
    ax.set_ylabel("Scores")
    ax.set_xlabel("Algorithm")
    ax.set_title(f"Best solutions for given method [{problem_name}]")

    fig.savefig(os.path.join(outputs_folder, f"mins_bar_{output_suffix}.png"))


if __name__ == "__main__":
    generate_graphs("a.txt", "viz_output", "a", "TSPA")
    generate_graphs("b.txt", "viz_output", "b", "TSPB")
    # generate_graphs("outputs/2025_01_26_19_12_51/output_a.txt", "viz_output", "a_new")
    # generate_graphs("outputs/2025_01_26_19_21_37/output_b.txt", "viz_output", "b_new")
