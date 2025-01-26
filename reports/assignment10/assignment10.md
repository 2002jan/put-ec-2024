# Assignment 10 - Own Method


## Authors
- Michał Kamiński 151969
- Jan Indrzejczak 152059

# Description of the problem

The travelling salesman problem (TSP) is a classic optimization problem. Given a list of cities and the distances between them, the task is to find the shortest possible route that visits each city exactly once and returns to the origin city. In this version of the problem, each city also has a cost of being visited, and we only need to select half of the cities.

As an input we received a list of coordinates of cities, along with the cost. To calculate the distance between cities we used Euclidean distance, and each city is represented as a number from 0 to n-1 (n-number of cities). The objective function is to find the route that minimizes the sum of distances between cities and the cost of visiting them.

# Description of the score improvement process

# Pseudocode of implemented algorithms

## Tournament based Hybrid evolutionary algorithm
```
Function RunHybridEvolutionary
    Initialize the starting population of 60 individuals (Random starting solution with SteepestDeltasLocalSearch run on each)
    Evaluate each individual in the population
    Sort the population by the fitness
    
    Until the running time is exceeded:
        Initialize new_population list
        
        Until length of new_population is less than 60
            Choose parent1 from the population at random
            
            Do crossover with a 70% chance:
                Choose parent2 from the population at random
                Perform crossover on parent1 and parent2 to create a child
                
            Perform mutation on the child If 
                - Crossover did not happen 
                OR
                - With a 70% chance after crossover
            
            Either run SteepestDeltasLocalSearch on the child or proceed without LS
            If the child already exists in the population go back to the beginning of the loop        
            
            Add child to new_population list
            
        Add current population to new_population and sort it by fitness
        
        Clear the population list
        Add the best individual from the merged new_population list and remove it from there
        
        Until the population length is less than 60
            Select 3 random individiuals from merged new_population list
            
            Find the best one from them, add it to the population and remove it from the new_population list
     
    Return the best individual from the population
```

Random move mutation
```
Function Mutate
    Generate a random move to be perfomed on an individual
    Perform the move
    Return the mutated individual
```

Partially Mapped Crossover
```
Function Crossover
    Select two random points in parent solution
    
    Initialize new_solution as a clone of parent1
    
    moved_1 = slice of parent1 from point1 to point2
    moved_2 = slice of parent2 from point1 to point2
    
    Copy slice from parent2 between selected points to new_solution between selected points
    
    Initialize changes_map as an map of points from moved slice to the points that it replaced in new_solution 
    
    For each point not replaced in new_solution
        If point is in changes_map
            SET new_value to changes_map[point]
            
            UNTIL slice moved from parent2 contains new_value
                SET new_value to changes_map[new_value]
                
            Update point with new_value
    
    Return new_solution
```

# Table of the results 

| Algorithm                                                                                                                          | TSPA                | TSPB                |
|------------------------------------------------------------------------------------------------------------------------------------|---------------------|---------------------|
| Random Start Two Edges Intra Steepest Deltas Iterated Local Search                                                                 | 69224 (69114-69469) | 43730 (43494-44095) |
| Hybrid Evolutionary Algorithm with LS after recombination                                                                          | 69893 (69593-70270) | 44897 (44336-45867) |
| Hybrid Evolutionary Algorithm with Tournament selection                                                                            | 69828 (69293-70221) | 44805 (44251-45867) |
| Hybrid Evolutionary Algorithm with Tournament selection, PMX crossover, Random replace mutation                                    | 69323 (69134-69729) | 43571 (43469-43674) |
| Hybrid Evolutionary Algorithm with Tournament selection, PMX crossover, Random move mutation                                       | 69286 (69114-69595) | 43557 (43478-43732) |
| Hybrid Evolutionary Algorithm followed by Iterated Local Search                                                                    | 69221 (69124-69423) | 43714 (43475-44151) |
| Hybrid Evolutionary Algorithm with Tournament selection followed by Iterated Local Search                                          | 69208 (69104-69410) | 43697 (43456-44180) |
| Hybrid Evolutionary Algorithm with Tournament selection, PMX crossover, Random replace mutation; followed by Iterated Local Search | 69181 (69107-69383) | 43493 (43448-43544) |
| Hybrid Evolutionary Algorithm with Tournament selection, PMX crossover, Random move mutation; followed by Iterated Local Search    | 69211 (69104-69382) | 43490 (43457-43539) |

## Acronyms
* RM -> Random replace mutation
* MM -> Random move mutation
* FC -> Keep Common Fill Ls Crossover
* PMX -> Partially mapped crossover
* HE -> Hybrid Evo
* TE -> Tournaments hybrid evo

## Plots

![mins_bar_a.png](assets%2Fmins_bar_a.png)
![ranges_a.png](assets%2Franges_a.png)
![mins_bar_b.png](assets%2Fmins_bar_b.png)
![ranges_b.png](assets%2Franges_b.png)

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
------Baseline------

Results for Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 69114
Max cost: 69469
Average cost: 69224

Best solution:
[102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 108, 18, 22, 146, 34, 160, 48, 54, 177, 10, 190, 4, 112, 84, 184, 131, 149, 65, 116, 43, 42, 181, 159, 193, 41, 139, 68, 46, 115, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 100, 26, 86, 75, 101, 1, 97, 152, 2, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 92, 129, 57, 179, 196, 81, 90, 165, 119, 40, 185, 55, 52, 106, 178, 49, 14, 144]



-----Pure Evo-------

Results for Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Hybrid Evolutionary algorithm
Min cost: 69593
Max cost: 70270
Average cost: 69893

Best solution:
[112, 4, 190, 10, 177, 54, 48, 34, 146, 22, 18, 108, 69, 159, 193, 41, 139, 115, 46, 68, 140, 93, 117, 0, 143, 183, 89, 23, 137, 186, 114, 15, 148, 9, 62, 102, 144, 14, 49, 178, 106, 52, 55, 185, 40, 165, 90, 81, 196, 179, 145, 78, 31, 56, 113, 175, 171, 16, 25, 44, 120, 82, 92, 57, 129, 2, 152, 97, 1, 101, 75, 86, 100, 26, 94, 63, 53, 180, 154, 135, 70, 127, 123, 162, 151, 133, 79, 80, 176, 51, 118, 59, 65, 116, 43, 42, 181, 160, 184, 84]

Results for Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Tournament hybrid Evolutionary algorithm
Min cost: 69293
Max cost: 70221
Average cost: 69828

Best solution:
[34, 160, 48, 54, 177, 10, 190, 4, 112, 84, 184, 42, 43, 116, 65, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 121, 100, 26, 86, 75, 101, 1, 97, 152, 2, 129, 57, 92, 82, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 179, 196, 81, 90, 165, 40, 185, 55, 52, 106, 178, 49, 14, 144, 102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 68, 46, 115, 139, 41, 193, 159, 69, 108, 18, 22, 146, 181]

Results for Steepest Deltas Random Replace Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm
Min cost: 69134
Max cost: 69729
Average cost: 69323

Best solution:
[79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 108, 18, 22, 146, 34, 160, 48, 54, 177, 10, 190, 4, 112, 84, 184, 35, 131, 149, 65, 116, 43, 42, 181, 159, 193, 41, 139, 68, 46, 115, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 100, 26, 86, 75, 101, 1, 97, 152, 2, 120, 44, 25, 16, 171, 175, 113, 31, 78, 145, 92, 129, 57, 179, 196, 81, 90, 165, 119, 40, 185, 55, 52, 106, 178, 49, 14, 144, 102, 62, 9, 148, 124, 94, 63]

Results for Steepest Deltas Random Move Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm
Min cost: 69114
Max cost: 69595
Average cost: 69286

Best solution:
[51, 118, 59, 115, 46, 68, 139, 41, 193, 159, 181, 42, 43, 116, 65, 149, 131, 35, 184, 84, 112, 4, 190, 10, 177, 54, 48, 160, 34, 146, 22, 18, 108, 140, 93, 117, 0, 143, 183, 89, 186, 23, 137, 176, 80, 79, 63, 94, 124, 148, 9, 62, 102, 144, 14, 49, 178, 106, 52, 55, 57, 129, 92, 179, 185, 40, 165, 90, 81, 196, 145, 78, 31, 56, 113, 175, 171, 16, 25, 44, 120, 2, 152, 97, 1, 101, 75, 86, 26, 100, 53, 180, 154, 135, 70, 127, 123, 162, 133, 151]



-----Mixed Evo------

Results for First Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 69124
Max cost: 69423
Average cost: 69221

Best solution:
[55, 52, 106, 178, 14, 144, 49, 102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 108, 18, 69, 68, 46, 115, 139, 41, 193, 159, 22, 146, 181, 34, 160, 48, 54, 177, 10, 190, 4, 112, 84, 35, 184, 42, 43, 116, 65, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 100, 26, 86, 75, 101, 1, 97, 152, 2, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 196, 81, 90, 165, 119, 40, 185, 179, 92, 129, 57]

Results for First Steepest Deltas Random Move Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 69104
Max cost: 69410
Average cost: 69208

Best solution:
[183, 89, 186, 23, 137, 176, 80, 79, 63, 94, 124, 148, 9, 62, 102, 144, 14, 49, 178, 106, 52, 55, 185, 40, 119, 165, 90, 81, 196, 179, 57, 129, 92, 145, 78, 31, 56, 113, 175, 171, 16, 25, 44, 120, 2, 152, 97, 1, 101, 75, 86, 26, 100, 53, 180, 154, 135, 70, 127, 123, 162, 133, 151, 51, 118, 59, 65, 116, 43, 42, 181, 160, 184, 35, 84, 112, 4, 190, 10, 177, 54, 48, 34, 146, 22, 159, 193, 41, 139, 115, 46, 68, 69, 18, 108, 140, 93, 117, 0, 143]

Results for First Steepest Deltas Random Replace Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 69107
Max cost: 69383
Average cost: 69181

Best solution:
[101, 1, 97, 152, 2, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 196, 81, 90, 165, 119, 40, 185, 179, 92, 129, 57, 55, 52, 106, 178, 49, 14, 144, 102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 108, 18, 22, 146, 34, 160, 48, 54, 177, 10, 190, 4, 112, 84, 184, 131, 149, 65, 116, 43, 42, 181, 159, 193, 41, 139, 68, 46, 115, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 100, 26, 86, 75]

Results for First Steepest Deltas Random Move Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 69104
Max cost: 69382
Average cost: 69211

Best solution:
[65, 59, 118, 51, 151, 133, 162, 123, 127, 70, 135, 154, 180, 53, 100, 26, 86, 75, 101, 1, 97, 152, 2, 120, 44, 25, 16, 171, 175, 113, 56, 31, 78, 145, 92, 129, 57, 179, 196, 81, 90, 165, 119, 40, 185, 55, 52, 106, 178, 49, 14, 144, 102, 62, 9, 148, 124, 94, 63, 79, 80, 176, 137, 23, 186, 89, 183, 143, 0, 117, 93, 140, 108, 18, 69, 68, 46, 115, 139, 41, 193, 159, 22, 146, 34, 48, 54, 177, 10, 190, 4, 112, 84, 35, 184, 160, 181, 42, 43, 116]
```

## TSPB

```
------Baseline------

Results for Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 43494
Max cost: 44095
Average cost: 43730

Best solution:
[1, 131, 121, 51, 90, 122, 135, 63, 40, 107, 133, 10, 147, 6, 188, 169, 132, 70, 3, 15, 145, 13, 195, 168, 139, 11, 138, 33, 160, 144, 104, 8, 21, 82, 111, 29, 0, 109, 35, 143, 106, 124, 62, 18, 55, 34, 170, 152, 183, 140, 4, 149, 28, 20, 60, 148, 47, 94, 66, 179, 185, 99, 130, 95, 86, 166, 194, 176, 180, 113, 114, 137, 127, 89, 103, 163, 187, 153, 81, 77, 141, 91, 61, 36, 177, 5, 78, 175, 142, 45, 80, 190, 136, 73, 54, 31, 193, 117, 198, 156]



-----Pure Evo-------

Results for Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Hybrid Evolutionary algorithm
Min cost: 44336
Max cost: 45867
Average cost: 44897

Best solution:
[190, 136, 73, 54, 31, 193, 117, 198, 156, 1, 16, 27, 38, 135, 63, 40, 107, 133, 122, 90, 121, 51, 147, 6, 188, 169, 132, 13, 70, 3, 15, 145, 195, 168, 43, 139, 11, 138, 33, 160, 29, 0, 109, 35, 143, 106, 124, 62, 18, 55, 34, 170, 152, 183, 140, 4, 149, 28, 59, 20, 60, 148, 47, 94, 66, 57, 172, 179, 185, 99, 130, 95, 86, 166, 194, 176, 113, 103, 127, 89, 163, 187, 153, 81, 77, 141, 91, 36, 61, 21, 82, 8, 104, 177, 5, 78, 175, 45, 162, 80]

Results for Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Tournament hybrid Evolutionary algorithm
Min cost: 44251
Max cost: 45867
Average cost: 44805

Best solution:
[5, 78, 175, 45, 80, 190, 136, 73, 54, 31, 193, 117, 198, 156, 1, 16, 27, 38, 63, 135, 122, 131, 121, 51, 90, 147, 6, 188, 169, 132, 13, 195, 168, 145, 15, 70, 3, 155, 184, 152, 170, 34, 55, 18, 62, 124, 106, 128, 95, 130, 183, 140, 4, 149, 28, 20, 60, 148, 47, 94, 66, 172, 179, 22, 99, 185, 86, 166, 194, 176, 113, 103, 127, 89, 163, 187, 153, 81, 77, 141, 91, 36, 61, 21, 82, 8, 104, 111, 35, 109, 0, 29, 160, 33, 11, 139, 138, 182, 25, 177]

Results for Steepest Deltas Random Replace Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm
Min cost: 43469
Max cost: 43674
Average cost: 43571

Best solution:
[156, 198, 117, 193, 31, 54, 73, 136, 190, 80, 45, 142, 175, 78, 5, 177, 36, 61, 91, 141, 77, 81, 153, 187, 163, 103, 89, 127, 137, 114, 113, 176, 194, 166, 86, 185, 95, 130, 99, 22, 179, 66, 94, 47, 148, 60, 20, 28, 149, 4, 140, 183, 152, 170, 34, 55, 18, 62, 124, 106, 143, 35, 109, 0, 29, 160, 33, 144, 111, 8, 104, 138, 11, 139, 168, 195, 13, 145, 15, 3, 70, 132, 169, 188, 6, 147, 90, 51, 121, 131, 135, 122, 133, 107, 40, 63, 38, 27, 16, 1]

Results for Steepest Deltas Random Move Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm
Min cost: 43478
Max cost: 43732
Average cost: 43557

Best solution:
[142, 45, 80, 190, 136, 73, 54, 31, 193, 117, 198, 156, 1, 16, 27, 38, 63, 40, 107, 122, 135, 131, 121, 51, 90, 147, 6, 188, 169, 132, 70, 3, 15, 145, 13, 195, 168, 139, 11, 138, 104, 8, 82, 111, 144, 33, 160, 29, 0, 109, 35, 143, 106, 124, 62, 18, 55, 34, 170, 152, 183, 140, 4, 149, 28, 20, 60, 148, 47, 94, 66, 179, 22, 99, 130, 95, 185, 86, 166, 194, 176, 113, 114, 137, 127, 89, 103, 163, 187, 153, 81, 77, 141, 91, 61, 36, 177, 5, 78, 175]



-----Mixed Evo------

Results for First Steepest Deltas Random Replace Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 43475
Max cost: 44151
Average cost: 43714

Best solution:
[63, 38, 27, 16, 1, 156, 198, 117, 193, 31, 54, 73, 136, 190, 80, 45, 142, 175, 78, 5, 177, 36, 61, 91, 141, 77, 81, 153, 187, 163, 103, 89, 127, 137, 114, 113, 176, 194, 166, 86, 95, 130, 99, 185, 179, 66, 94, 47, 148, 60, 20, 28, 149, 4, 140, 183, 152, 170, 34, 55, 18, 62, 124, 106, 143, 35, 109, 0, 29, 111, 82, 8, 104, 144, 160, 33, 138, 11, 139, 168, 195, 13, 145, 15, 3, 70, 132, 169, 188, 6, 147, 191, 90, 51, 121, 131, 135, 122, 107, 40]

Results for First Steepest Deltas Random Move Mutation Keep Common Fill LS Crossover with Greedy Regret Heuristic with weighted 2-Regret Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 43456
Max cost: 44180
Average cost: 43697

Best solution:
[122, 107, 40, 63, 38, 27, 16, 1, 156, 198, 117, 193, 31, 54, 73, 136, 190, 80, 45, 142, 175, 78, 5, 177, 36, 61, 91, 141, 77, 81, 153, 187, 163, 103, 89, 127, 137, 114, 113, 176, 194, 166, 86, 185, 95, 130, 99, 179, 66, 94, 47, 148, 60, 20, 28, 149, 4, 140, 183, 152, 170, 34, 55, 18, 62, 124, 106, 143, 35, 109, 0, 29, 111, 82, 21, 8, 104, 144, 160, 33, 138, 11, 139, 168, 195, 13, 145, 15, 3, 70, 132, 169, 188, 6, 147, 90, 51, 121, 131, 135]

Results for First Steepest Deltas Random Replace Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 43448
Max cost: 43544
Average cost: 43493

Best solution:
[78, 175, 142, 45, 80, 190, 136, 73, 54, 31, 193, 117, 198, 156, 1, 16, 27, 38, 63, 40, 107, 133, 122, 135, 131, 121, 51, 90, 147, 6, 188, 169, 132, 70, 3, 15, 145, 13, 195, 168, 139, 11, 138, 33, 160, 144, 104, 8, 111, 29, 0, 109, 35, 143, 106, 124, 62, 18, 55, 34, 170, 152, 183, 140, 4, 149, 28, 20, 60, 148, 47, 94, 66, 179, 22, 99, 130, 95, 185, 86, 166, 194, 176, 113, 114, 137, 127, 89, 103, 163, 187, 153, 81, 77, 141, 91, 61, 36, 177, 5]

Results for First Steepest Deltas Random Move Mutation Partially mapped crossover Tournament hybrid Evolutionary algorithm then Random Start Two Edges Intra Steepest Deltas Iterated Local Search
Min cost: 43457
Max cost: 43539
Average cost: 43490

Best solution:
[8, 104, 144, 160, 33, 138, 11, 139, 168, 195, 13, 145, 15, 3, 70, 132, 169, 188, 6, 147, 90, 51, 121, 131, 135, 122, 107, 40, 63, 38, 27, 16, 1, 156, 198, 117, 193, 31, 54, 73, 136, 190, 80, 45, 142, 175, 78, 5, 177, 36, 61, 91, 141, 77, 81, 153, 187, 163, 103, 89, 127, 137, 114, 113, 176, 194, 166, 86, 185, 95, 130, 99, 22, 179, 66, 94, 47, 148, 60, 20, 28, 149, 4, 140, 183, 152, 170, 34, 55, 18, 62, 124, 106, 143, 35, 109, 0, 29, 111, 82]
```


# Plots of the best results

## TSPA
![TSPA_te_mm_pmx_ils.csv.png](assets%2FTSPA_te_mm_pmx_ils.csv.png)


## TSPB
![TSPB_te_mm_pmx_ils.csv.png](assets%2FTSPB_te_mm_pmx_ils.csv.png)


# Source code

- [Github repository](https://github.com/2002jan/put-ec-2024)

# Conclusions