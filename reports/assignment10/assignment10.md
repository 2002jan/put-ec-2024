# Assignment 10 - Own Method


## Authors
- Michał Kamiński 151969
- Jan Indrzejczak 152059

# Description of the problem

The travelling salesman problem (TSP) is a classic optimization problem. Given a list of cities and the distances between them, the task is to find the shortest possible route that visits each city exactly once and returns to the origin city. In this version of the problem, each city also has a cost of being visited, and we only need to select half of the cities.

As an input we received a list of coordinates of cities, along with the cost. To calculate the distance between cities we used Euclidean distance, and each city is represented as a number from 0 to n-1 (n-number of cities). The objective function is to find the route that minimizes the sum of distances between cities and the cost of visiting them.

# Description of the score improvement process

# Pseudocode of implemented algorithms

```
Tournament hybrid
```

```
mutation operator
```

# Table of the results 

| Algorithm                                                                                 | TSPA                | TSPB                |
|-------------------------------------------------------------------------------------------|---------------------|---------------------|
| Random Start Two Edges Intra Steepest Deltas Iterated Local Search                        | 69340 (69122-69531) | 43647 (43507-43877) |
| Hybrid Evolutionary Algorithm with LS after recombination                                 | 69831 (69293-70248) | 45231 (44659-45867) |
| Hybrid Evolutionary Algorithm with Tournament selection                                   | 69840 (69357-70193) | 45199 (44497-45904) |
| Hybrid Evolutionary Algorithm followed by Iterated Local Search                           | 69288 (69104-69654) | 43744 (43503-44097) |
| Hybrid Evolutionary Algorithm with Tournament selection followed by Iterated Local Search | 69212 (69095-69337) | 43886 (43498-45231) |

## Results of previous algorithms

| Algorithm                                                                | TSPA                   | TSPB                |
|--------------------------------------------------------------------------|------------------------|---------------------|
| Random Algorithm                                                         | 225467                 | 193417              |
| Nearest Neighbor (Add at End)                                            | 83182                  | 52319               |
| Nearest Neighbor (Insert Anywhere)                                       | 71179                  | 44417               |
| Greedy Cycle                                                             | 72636 (71488-74410)    | 51401 (49001-57324) |
| Greedy Regret Heuristic with 2-Regret                                    | 116681 (108804-123447) | 70265 (65043-76325) |
| Greedy Regret Heuristic with Weighted 2-Regret                           | 72148 (71108-73718)    | 50997 (47144-56747) |
| Random Start Two Edges Intra Steepest Candidate                          | 79763 (74876-84144)    | 51500 (47433-58226) |
| Random Start Two Edges Intra Steepest                                    | 75172 (72784-80372)    | 49635 (47325-52654) |
| Random Start Two Nodes Intra Greedy                                      | 86727 (82039-95867)    | 61477 (53396-67230) |
| Random Start Two Edges Intra Greedy                                      | 74035 (77907-82039)    | 48390 (45665-51760) |
| Greedy Start Two Nodes Intra Greedy                                      | 71599 (70602-72778)    | 45331 (43826-51911) |
| Greedy Start Two Edges Intra Greedy                                      | 71335 (70004-72452)    | 44898 (43790-50892) |
| Random Start Two Nodes Intra Steepest                                    | 88618 (81178-98102)    | 63387 (56112-73195) |
| Greedy Start Two Nodes Intra Steepest                                    | 71936 (71041-73353)    | 45355 (43862-51147) |
| Greedy Start Two Edges Intra Steepest                                    | 71677 (70397-72984)    | 45008 (43958-50901) |
| Random Start Two Edges Intra Steepest Candidate                          | 79763 (74876-84144)    | 51500 (47433-58226) |
| Random Start Two Edges Intra Steepest                                    | 75326 (72938-80126)    | 49725 (46957-52832) |
| Random Start Two Edges Intra Steepest Deltas                             | 74207 (71342-78723)    | 49160 (46761-52674) |
| Random Start Two Edges Intra Steepest Multiple Start Local Search        | 71299 (70817-71812)    | 45970 (45235-46595) |
| Random Start Two Edges Intra Steepest Deltas Multiple Start Local Search | 71700 (70758-72325)    | 46101 (45578-46676) |
| Greedy Regret Heuristic with weighted 2-Regret Random Destroy LNS        | 70987 (70024-72625)    | 46034 (44398-48968) |
| Greedy Regret Heuristic with weighted 2-Regret Random Destroy LNSw       | 69720 (69378-70168)    | 44095 (43509-44602) |
| Hybrid Evolutionary Algorithm with without LS after recombination        | 70094 (69369-70618)    | 45185 (44236-46196) |

# Raw results

## TSPA

```
Results for Hybrid Evolutionary Algorithm with Tournament selection
Min cost: 69357
Max cost: 70193
Average cost: 69840

Time took for 20 runs: 41.92043030s, time per run: 2096021μs

Best solution:
[52, 106, 178, 49, 14, 144, 102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 89, 183, 143, 0, 117, 93, 140, 68, 46, 115, 139, 41, 193, 159, 69, 108, 18, 22, 146, 34, 48, 54, 177, 10, 190, 4, 112, 84, 35, 184, 160, 181, 42, 43, 116, 65, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 121, 100, 26, 86, 75, 101, 1, 97, 152, 2, 129, 57, 92, 82, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 179, 196, 81, 90, 165, 40, 185, 55]


Results for Hybrid Evolutionary algorithm followed by Iterated Local Search
Min cost: 69104
Max cost: 69654
Average cost: 69288

Time took for 20 runs: 80.05665870s, time per run: 4002832μs

Best solution:
[178, 49, 14, 144, 102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 108, 18, 69, 68, 46, 115, 139, 41, 193, 159, 22, 146, 34, 48, 54, 177, 10, 190, 4, 112, 84, 35, 184, 160, 181, 42, 43, 116, 65, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 100, 26, 86, 75, 101, 1, 97, 152, 2, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 92, 129, 57, 179, 196, 81, 90, 165, 119, 40, 185, 55, 52, 106]

Results for Hybrid Evolutionary Algorithm with Tournament selection followed by Iterated Local Search
Min cost: 69095
Max cost: 69337
Average cost: 69212

Time took for 20 runs: 82.05872280s, time per run: 4102936μs

Best solution:
[92, 179, 185, 40, 119, 165, 90, 81, 196, 145, 78, 31, 56, 113, 175, 171, 16, 25, 44, 120, 2, 152, 97, 1, 101, 75, 86, 26, 100, 53, 180, 154, 135, 70, 127, 123, 162, 133, 151, 51, 118, 59, 65, 116, 43, 42, 184, 35, 84, 112, 4, 190, 10, 177, 54, 48, 160, 34, 181, 146, 22, 159, 193, 41, 139, 115, 46, 68, 69, 18, 108, 140, 93, 117, 0, 143, 183, 89, 186, 23, 137, 176, 80, 79, 63, 94, 124, 148, 9, 62, 102, 144, 14, 49, 178, 106, 52, 55, 57, 129]


```

## TSPB

```
Results for Hybrid Evolutionary Algorithm with Tournament selection
Min cost: 44497
Max cost: 45904
Average cost: 45199

Time took for 20 runs: 41.79518410s, time per run: 2089759μs

Best solution:
[89, 127, 103, 113, 176, 194, 166, 86, 95, 130, 99, 185, 179, 172, 57, 66, 94, 47, 148, 60, 20, 59, 28, 149, 4, 140, 183, 152, 170, 34, 55, 18, 62, 124, 106, 143, 35, 109, 0, 29, 160, 33, 138, 11, 139, 43, 168, 195, 145, 15, 3, 70, 13, 132, 169, 188, 6, 147, 51, 121, 131, 90, 191, 10, 133, 122, 135, 63, 38, 27, 16, 1, 156, 198, 117, 193, 31, 54, 73, 136, 190, 80, 45, 175, 78, 5, 177, 104, 8, 82, 21, 61, 36, 91, 141, 77, 81, 153, 187, 163]

```


# Plots of the results

## TSPA

![TSPA_Tournament_hybrid_min.csv.png](assets%2FTSPA_Tournament_hybrid_min.csv.png)
![TSPA_Hybrid_and_ils_min.csv.png](assets%2FTSPA_Hybrid_and_ils_min.csv.png)
![TSPA_Tournament_hybrid_and_ils_min.csv.png](assets%2FTSPA_Tournament_hybrid_and_ils_min.csv.png)

## TSPB

![TSPB_Tournament_hybrid_min.csv.png](assets%2FTSPB_Tournament_hybrid_min.csv.png)


# Source code

- [Github repository](https://github.com/2002jan/put-ec-2024)

# Conclusions