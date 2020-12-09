# Challenge 9

The challenge can be found [here][1]

The `input` is a preamble of 25 numbers followed by a list of numbers which must be composed of 2
numbers in the previous 25.

### Tasks

1. Based on the input which number does not follow the rule of being composed of 2 numbers from the 25
preceding it
2. Find a contiguous set of numbers in the input which sum to the answer from task 1 and find the sum of
the min and max values from this set

### Solutions

##### 1

This uses a ring buffer for storing the previous values and the valid combos associated with them. Each
time a line is read be add a vec to the combo buffer containing all valid combiniations of that value
and preceding values. We also pop the latest value from each combo buffer each line read to reflect no
longer valid combos as previous values go "out of scope".

This *could* be more memory efficient for larger inputs of preambles though I suspect tweaking to
prevent the `vec.slice().to_vec()` would be required to make this truly useful...

```bash
rustc solution.rs
./solution input
```

##### 2

```bash
rustc solution2.rs
./solution2 input
```

[1]: <https://adventofcode.com/2020/day/9> "Advent of Code day 9 challenge"
