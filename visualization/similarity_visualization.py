import pandas as pd
import numpy as np
import sys
import os
import matplotlib.pyplot as plt
from scipy.stats import pearsonr
def generate_graph(dir_path, file_path):
    # Load the data
    data = pd.read_csv(dir_path + file_path, header=None)

    objective = data.iloc[-1, -1]
    # remove last row
    data = data.iloc[:-1, :]
    # cast all elements to int
    if "best" in file_path:
        data = data.astype(int)
    else:
        data = data.astype(float)
    lst_obj = list(data.iloc[:, 0])
    lst_sim = list(data.iloc[:, 1])
    # print(len(set(lst_obj)))

    correlaction_coeff = pearsonr(lst_obj, lst_sim).statistic


    ftype = "_".join(file_path.split("_")[-3:])[:-4]
    plt.figure(figsize=(10, 10))
    plt.scatter(lst_obj, lst_sim)
    plt.title(f"{ftype}: {round(correlaction_coeff,3)}")
    plt.xlabel("Objective function value")
    plt.ylabel("Similarity")

    plt.savefig(f'{dir_path}/plots/{ftype}.png')

#     create scatter plot



if __name__ == '__main__':
    # parse arguments
    dir_path = sys.argv[1]
    files = os.listdir(dir_path)
    # mkdir plots
    if not os.path.exists(f'{dir_path}\plots'):
        os.makedirs(f'{dir_path}\plots')

    for file in files:
        generate_graph(dir_path, file)