# Game of Life — Hand Counted Neighbor Counts

## 1. Middle Vertical Line

Three live cells are placed vertically in the middle of the grid:

- (4, 10)
- (5, 10)
- (6, 10)

Each number below represents the number of live neighbors for that cell.

```text
1 1 1
2 1 2
3 2 3
2 1 2
1 1 1
## 2. Vertical Line Along the Left Edge

Three live cells are placed vertically starting at the top-left corner:

- (0, 0)
- (1, 0)
- (2, 0)

The neighbor counts are:

```text
1 2
2 3
1 2
1 1

## 3. Comparison

The hand counted should match the results from the print neighbor counts function.

The middle vertical line shows how neighboring cells are counted away from the edges, 
while the left-edge line shows how the program handles boundary cells.
