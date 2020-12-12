# Challenge 12

The challenge can be found [here][1]

The `input` is a list of instructions for some ship with each composed of a letter and number. These are
- N(orth), E(ast), S(outh), W(est) representing a direction to travel with the value being the distance
- R(ight), L(eft) representing a direction to turn with the value being the number of degrees to turn
- F(orward) representing a move in the current facing direction with the value being the distance

Notes:
- manhattan distance is the sum of the absolute distance of each axis from the origin i.e.  
`abs(origin - ship.x) + abs(origin - ship.y)`
- In task 1 the instructions are applied directly to the ship
- In task 2 the instructions are applied to a waypoint (vector) which the ship will travel in when the
F(orward) instruction is applied. N, E, S, W moves the waypoint in the specified direction and L, R rotates the
the waypoint about the the ship.

### Tasks

1. What is the manhattan distance of the ship from the origin once all instructions are followed
2. As task 1 but with instructions applying to the waypoint

### Solutions

##### 1 and 2

```bash
rustc solution.rs
./solution input
```

### Notes

Would make more sense to do rotations with linear algebra (e.g. `ndarray`).

Moved most rust code into the `rustlib` folder under appropriate modules.

[1]: <https://adventofcode.com/2020/day/12> "Advent of Code day 12 challenge"
