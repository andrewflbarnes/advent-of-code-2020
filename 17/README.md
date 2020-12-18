# Challenge 17

The challenge can be found [here][1]

Conway's game of life in 3-space and 4-space!

Input is the starting state - a 2 dimensional slice in the n-space where `#` is active amd `.` is
inactive. Other than this starting slice all other points are initialised to an inactive (`.` ) state.

Each cycle state is updated as below
- if a cell is active and 2 or 3 neighbour cells are active it remains active, otherwise it becomes
inactive
- if a cell is inactive and exactly 3 neighbour cells are active it becomes active, otherwise it
remains inactive

### Tasks

1. How many cells are active in 3-space after 6 cycles
1. How many cells are active in 4-space after 6 cycles

### Solutions

##### 1 and 2

```bash
rustc solution.rs
./solution input
```

[1]: <https://adventofcode.com/2020/day/17> "Advent of Code day 17 challenge"
