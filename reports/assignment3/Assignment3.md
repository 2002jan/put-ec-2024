# Assignment 3 - Local Search 

## Authors 
- Michał Kamiński 151969
- Jan Indrzejczak 152059


# Desciption of the problem

The travelling salesman problem (TSP) is a classic optimization problem. Given a list of cities and the distances between them, the task is to find the shortest possible route that visits each city exactly once and returns to the origin city. In this version of the problem, each city also has a cost of being visited, and we only need to select half of the cities.

As an input we received a list of coordinates of cities, along with the cost. To calculate the distance between cities we used Euclidean distance, and each city is represented as a number from 0 to n-1 (n-number of cities). The objective function is to find the route that minimizes the sum of distances between cities and the cost of visiting them.

# Pseudocode of all implemented algorithms 

# Reults of computational experiments 
## TSPA

```
Results for Greedy Local Search, Two-nodes exchange, Random start
Min cost: 80429
Max cost: 92439
Average cost: 86715

Results for Greedy Local Search, Two-edges exchange, Random start
Min cost: 70923
Max cost: 77820
Average cost: 74058

Results for Greedy Local Search, Two-nodes exchange, Greedy start
Min cost: 70598
Max cost: 72746
Average cost: 71596

Results for Greedy Local Search, Two-edges exchange, Greedy start
Min cost: 69938
Max cost: 72346
Average cost: 71322

Results for Steepest Local Search, Two-nodes exchange, Random start
Min cost: 79780
Max cost: 96499
Average cost: 88543

Results for Steepest Local Search, Two-edges exchange, Random start
Min cost: 72944
Max cost: 78567
Average cost: 75320

Results for Steepest Local Search, Two-nodes exchange, Greedy start
Min cost: 71041
Max cost: 73353
Max cost: 73353
Average cost: 71936

Results for Steepest Local Search, Two-edges exchange, Greedy start
Min cost: 70397
Max cost: 72984
Average cost: 71677

```

## TSPB

```
Results for Greedy Local Search, Two-nodes exchange, Random start
Min cost: 54949
Max cost: 67564
Average cost: 61174

Results for Greedy Local Search, Two-edges exchange, Random start
Min cost: 45996
Max cost: 51166
Average cost: 48264

Results for Greedy Local Search, Two-nodes exchange, Greedy start
Min cost: 43826
Max cost: 51565
Average cost: 45321

Results for Greedy Local Search, Two-edges exchange, Greedy start
Min cost: 43790
Max cost: 51225
Average cost: 44916

Results for Steepest Local Search, Two-nodes exchange, Random start
Min cost: 55431
Max cost: 72484
Average cost: 63291

Results for Steepest Local Search, Two-edges exchange, Random start
Min cost: 47416
Max cost: 52303
Average cost: 49724

Results for Steepest Local Search, Two-nodes exchange, Greedy start
Min cost: 43862
Max cost: 51147
Average cost: 45355

Results for Steepest Local Search, Two-edges exchange, Greedy start
Min cost: 43958
Max cost: 50901
Average cost: 45008

```


# Plots of the results

## TSPA


## TSPB


# Best solutions as a list of nodes

## TSPA

## TSPB

# Source code

- [Github repository](https://github.com/2002jan/put-ec-2024)


# Conclusions