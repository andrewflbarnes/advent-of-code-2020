# Challenge 11

The challenge can be found [here][1]

The `input` is a seating layout with floor `.` and seats `L`. Each tick s seat's state changes based on
a few rules:
- If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
- If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
- Otherwise, the seat's state does not change.

This is essentially a Conway's game of life puzzle.

### Tasks

1. How many seats are occupied once a steady state is reached?
2. As above but rather than checking adjacent seats check seats generally in each of the 8 directions

### Solutions

##### 1

```bash
rustc solution.rs
./solution input
```

##### 2

```bash
rustc solution.rs
./solution input
```

[1]: <https://adventofcode.com/2020/day/11> "Advent of Code day 11 challenge"
