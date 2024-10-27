import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import sys
import os
from math import ceil


def visualize_path(csv_file, node_indices, algorithm, output_dir):
    data = pd.read_csv(csv_file, sep=';', header=None)
    data.columns = ['x', 'y', 'cost']

    x_max = data['x'].max()
    y_max = data['y'].max()

    aspect_ration = x_max / y_max

    selected_nodes = data.iloc[node_indices]

    all_indices = set(range(len(data)))
    unused_indices = list(all_indices - set(node_indices))
    unused_nodes = data.iloc[unused_indices]

    norm = plt.Normalize(vmin=selected_nodes['cost'].min(), vmax=selected_nodes['cost'].max())
    cmap = sns.color_palette("coolwarm", as_cmap=True)

    plot_size = (ceil(8 * aspect_ration), 8) if aspect_ration >= 1 else (8, ceil(8 / aspect_ration))

    plt.figure(figsize=plot_size)

    scatter = plt.scatter(
        selected_nodes['x'], 
        selected_nodes['y'], 
        c=selected_nodes['cost'], 
        cmap=cmap, 
        s=100, 
        edgecolor='black', 
        norm=norm,
        label="Selected Nodes"
    )

    plt.scatter(
        unused_nodes['x'], 
        unused_nodes['y'], 
        c='gray', 
        s=50, 
        edgecolor='black', 
        label="Unused Nodes"
    )

    # Plot the Hamiltonian cycle (connect the selected nodes)
    for i in range(len(node_indices)):
        start_node = selected_nodes.iloc[i]
        end_node = selected_nodes.iloc[(i + 1) % len(selected_nodes)]  # Connect to the next, wrap at end
        plt.plot([start_node['x'], end_node['x']], [start_node['y'], end_node['y']], color='gray', linewidth=1.5)

    # Add a color bar for the node costs (only for selected nodes)
    plt.colorbar(scatter, label="Node Cost")

    plt.xlabel('X Coordinate')
    plt.ylabel('Y Coordinate')
    plt.title(f'{algorithm}')

    # Save the plot to a file (optional)
    # add timestamp to the filename

    # date = pd.to_datetime('today').strftime('%Y-%m-%d-%H-%M-%S')

    ax = plt.gca()
    ax.set_aspect('equal', adjustable='box')

    plt.savefig(f'{output_dir}/{algorithm}.png')


if __name__ == '__main__':

    # get the filename from args
    filename = sys.argv[1]
    if "min" not in filename:
        exit()
    algorithm = filename.split('\\')[-1].split('_score')[0].capitalize()
    node_indices = pd.read_csv(f'{filename}', header=None)

    csv_file = sys.argv[2] if len(sys.argv) > 2 else 'data/TSPA.csv'

    if len(sys.argv) > 3:
        output_directory = sys.argv[3]
    else:
        if not os.path.exists('plots'):
            os.makedirs('plots')
        output_directory = 'plots'

    node_indices = node_indices.iloc[:, 0].tolist()

    visualize_path(csv_file, node_indices, algorithm, output_directory)


