# Assignment 8 - Global Convexity tests

## Authors 
- Michał Kamiński 151969
- Jan Indrzejczak 152059

# Pseudocode of all implemented methods

```
Function SimilarityToBestSolution
    Compute the best solution and its objective function value using the Iterated Local Search algorithm
    Initialize the vector of pairs to store the similarity measure and the objective function value
    
    Repeat 1000 times
        Generate a random solution
        Run the Greedy Steepest local search algorithm on the random solution
        Compute the objective function value
        Compute the similarity to the best solution using either the common edges or the common nodes measure
        Store the similarity measure and the objective function value in the vector
        
    Return the vector
```

```
Function SimilarityToAllSolutions
    Initialize the vector of pairs to store the similarity measure and the objective function value
    Initialize the vector of solutions
    Initialize the vector of objective function values
    
    Repeat 1000 times
        Generate a random solution
        Run the Greedy Steepest local search algorithm on the random solution
        Compute the objective function value
        Store the solution in the vector of solutions
        Store the objective function value in the vector of objective function values
    
    For each solution in the vector of solutions
        Compute the average similarity to all other solutions using either the common edges or the common nodes measure
        Store the similarity measure and the objective function value in the pairs vector
    
    Return the vector of pairs
```
Common Nodes similarity measure:
```
Function evaluateCommonNodes
    Input: solution1, solution2
    Cast solution1 and solution2 to sets
    Compute the length of the intersection of the sets
    Return the length
```

Common Edges similarity measure:
```
Function evaluateCommonEdges
    Input: solution1, solution2
    Initialize a counter of common edges
    Create a set of all edges and their reversed counterparts from solution1
    Iterate through the edges of solution2
        if the edge is in the set of edges from solution1
            increment the counter
    Return the counter
```
# Plots of the results

## TSPA

### Comparison to the best solution (generated using the Iterated Local Search algorithm)
Correlation coefficients:
- Common Edges: -0.546
- Common Nodes: -0.674

![TSPA_common_edges_best.png](assets%2FTSPA_common_edges_best.png)
<div style="page-break-after: always"></div>

![TSPA_common_nodes_best.png](assets%2FTSPA_common_nodes_best.png)

### Comparison to all other solutions
Correlation coefficients:
- Common Edges: -0.637
- Common Nodes: -0.485

![common_edges_avg.png](..%2F..%2Foutputs%2F2025_01_08_18_18_52%2Fplots%2Fcommon_edges_avg.png)
<div style="page-break-after: always"></div>

![common_nodes_avg.png](..%2F..%2Foutputs%2F2025_01_08_18_18_52%2Fplots%2Fcommon_nodes_avg.png)

## TSPB

### Comparison to the best solution (generated using the Iterated Local Search algorithm)
Correlation coefficients:
- Common Edges: -0.652
- Common Nodes: -0.678

![common_edges_best.png](assets%2Fcommon_edges_best.png)
<div style="page-break-after: always"></div>

![common_nodes_best.png](assets%2Fcommon_nodes_best.png)
### Comparison to all other solutions
Correlation coefficients:
- Common Edges: -0.755
- Common Nodes: -0.538

![common_edges_avg.png](assets%2Fcommon_edges_avg.png)
<div style="page-break-after: always"></div>

![common_nodes_avg.png](assets%2Fcommon_nodes_avg.png)

### Table of correlation coefficients

|      | Best Node | Best Edge | Avg Node | Avg Edge |
|------|-----------|-----------|----------|----------|
| TSPA | -0.674    | -0.546    | -0.485   | -0.637   |
| TSPB | -0.678    | -0.652    | -0.538   | -0.755   |
# Raw results

## TSPA

```
Best Cost: 69205
Comparing to: Best Solution, Similarity measure: Common Nodes, Average Similarity: 88

Best Cost: 69583
Comparing to: Best Solution, Similarity measure: Common Edges, Average Similarity: 52

Comparing to: All other solutions, Similarity measure: Common Nodes, Average Similarity: 87.750984

Comparing to: All other solutions, Similarity measure: Common Edges, Average Similarity: 45.81864

```
## TSPB
```
Best Cost: 43625
Comparing to: Best Solution, Similarity measure: Common Nodes, Average Similarity: 83

Best Cost: 43515
Comparing to: Best Solution, Similarity measure: Common Edges, Average Similarity: 50

Comparing to: All other solutions, Similarity measure: Common Nodes, Average Similarity: 83.21918

Comparing to: All other solutions, Similarity measure: Common Edges, Average Similarity: 43.12958
```
# Source code

- [Github repository](https://github.com/2002jan/put-ec-2024)


# Conclusions
In all cases the correlation is negative. This means the lower the objective function value, the higher the similarity to either the best solution or on average to all other solutions.
We can interpret these results of similarity to best solution in the following way: if the solutions obtained by an objectively worse method have better scores, they tend to be closely related in terms of structure to that generated by an objectively better method. 
The strongest correlation occurs when calculating the average similarity to all other solutions using edges, interestingly this plot does not show the highest values of similarity (up to 50). The highest values of similarity occur when comparing solutions to some objectively best solution (similarity reaching 92).