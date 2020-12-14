# Challenge 14

The challenge can be found [here][1]

This `input` is a set of instructions for a program. There are two "classes" of instructions:
- `mem[i] = n` - set memory location `i` to value `n`
- `mask = ...` - set the bit mask as described


For task 1 the bit mask determines how values are "adjusted" before being written to memory based on the
character at each position:
- when `0` the bit at that position is changed to `0`
- when `1` the bit at that position is changed to `1`
- when `X` the bit at that position stays the same

For task 2 the bit mask determines how memory locations are "adjusted" before having values written to
them based on the character at each position:
- when `0` the bit at that position is left as is
- when `1` the bit at that position is changed to `1`
- when `X` the bit at that position is considered floating and all memory locations apply

The `X` condition for task 2 will result in multiple memory positions being set. More generally for each
`mem` instruction `2^n` memory locations will be written to where `n` is the number of occurrences of `X`.

### Tasks

1. What is the sum of the values in memory after all instructions are applied
2. As 1 but with the modified instruction interpretation

### Solutions

##### 1

```bash
rustc solution.rs
./solution input
```


[1]: <https://adventofcode.com/2020/day/14> "Advent of Code day 14 challenge"
