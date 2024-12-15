# Assignment 8 - Global Convexity tests

## Authors 
- Michał Kamiński 151969
- Jan Indrzejczak 152059

# Pseudocode of all implemented methods


# Plots of the results

## TSPA

### Comparison to the best solution (generated using the Iterated Local Search algorithm)
Correlation coefficients:
- Common Edges: -0.578
- Common Nodes: -0.678

![TSPA_common_edges_best.png](assets%2FTSPA_common_edges_best.png)
![TSPA_common_nodes_best.png](assets%2FTSPA_common_nodes_best.png)
### Comparison to all other solutions
Correlation coefficients:
- Common Edges: -0.636
- Common Nodes: -0.512

![TSPA_common_edges_avg.png](assets%2FTSPA_common_edges_avg.png)
![TSPA_common_nodes_avg.png](assets%2FTSPA_common_nodes_avg.png)

## TSPB

### Comparison to the best solution (generated using the Iterated Local Search algorithm)
Correlation coefficients:
- Common Edges: -0.658
- Common Nodes: -0.688
![common_edges_best.png](assets%2Fcommon_edges_best.png)
![common_nodes_best.png](assets%2Fcommon_nodes_best.png)

### Comparison to all other solutions
Correlation coefficients:
- Common Edges: -0.774
- Common Nodes: -0.557
![common_edges_avg.png](assets%2Fcommon_edges_avg.png)
![common_nodes_avg.png](assets%2Fcommon_nodes_avg.png)

# Raw results

## TSPA

```
Best Cost: 69379
Comparing to: Best Solution, Similarity measure: Common Nodes, Average Similarity: 87

Best Cost: 69625
Comparing to: Best Solution, Similarity measure: Common Edges, Average Similarity: 50

Comparing to: All other solutions, Similarity measure: Common Nodes, Average Similarity: 87

Comparing to: All other solutions, Similarity measure: Common Edges, Average Similarity: 45
```
## TSPB
```
Best Cost: 43584
Comparing to: Best Solution, Similarity measure: Common Nodes, Average Similarity: 85

Best Cost: 43924
Comparing to: Best Solution, Similarity measure: Common Edges, Average Similarity: 50

Comparing to: All other solutions, Similarity measure: Common Nodes, Average Similarity: 82

Comparing to: All other solutions, Similarity measure: Common Edges, Average Similarity: 42

```
# Source code

- [Github repository](https://github.com/2002jan/put-ec-2024)


# Conclusions
