# Challenge 10

The challenge can be found [here][1]

The `input` is a set of power adapter "joltages". Adapters may be "plugged in" to sources which are up to
3 jolts lower than the current one. e.g. a 5 jolt adapter may be plugged into a 5, 4, 3 or 2 jolt source
(like another adapter) but not a 1 or 6 jolt source.

Not included in the input file are
- the initial 0 jolt source (like a power socket)
- the final device which is always 3 jolts higher than the highest power adapter

### Tasks

1. Order the adapaters so they can all be plugged in in a row between the source and the device. Calculate
the product of the number of 1 jolt increments and 3 jolt increments.
2. ???

### Solutions

##### 1

```bash
rustc solution.rs
./solution input
```

[1]: <https://adventofcode.com/2020/day/10> "Advent of Code day 10 challenge"
