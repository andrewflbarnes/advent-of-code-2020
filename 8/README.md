# Challenge 8

The challenge can be found [here][1]

The `input` is a set of instructions for a bootloader - each line is one of `acc`, `jmp` or `nop` where
- `acc` updates the accumulator by the proceding signed value
- `jmp` jumps to the instruction relative to itself defined by the proceding signed value (`jmp +1` is
the next instruction, `jmp -1` is the previous, etc.)
- `nop` is no-op.

### Tasks

1. Based on the input rules, what is the value of the accumulator before any instruction is executed a
second time
2. A single instruction change from `jmp` to `nop` or `nop` to `jmp` may be made which allows the program
to complete (reach the final intruction). What is the value of the accumulator when the change is made

### Solutions

##### 1

```bash
rustc solution.rs
./solution input
```

##### 2

```bash
rustc solution2.rs
./solution2 input
```

### Notes

##### Rust

Because a lot of processing was abstracted into the `State` struct there was minimal similarity between the
processing for solutions 1 and 2. The common features (structs, input parsing) have been separated into a
separate `operations` module.

[1]: <https://adventofcode.com/2020/day/8> "Advent of Code day 8 challenge"
