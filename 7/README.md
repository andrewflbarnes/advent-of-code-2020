# Challenge 2

The challenge can be found [here][1]

The `input` is a set of rules defining bag colors and what other colored bags they must contains. These
rules apply recursively e.g. if a red bag contains 3 blue blags and a green bag contains 3 red bags then
a green bag contains 3 red bags and the 9 (3*3) blue bahs within them for a total of 12 bags.

### Tasks

1. Based on the input rules, how many different bag colors may eventually contain a "shiny gold" bag.
2. Based on the input rules how bags does a "shiny gold" bag contain.
groups

### Solutions

##### 1 and 2

Due to similarity in processing a single solultion is provided:
```bash
rustc solution.rs
./solution input
```

[1]: <https://adventofcode.com/2020/day/7> "Advent of Code day 7 challenge"
