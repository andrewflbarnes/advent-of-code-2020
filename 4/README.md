# Challenge 2

The challenge can be found [here][1]

The `input` is a set of passports. Fields are delimited by spaces or newlines. Passports are delimited by
empty lines.

Rules on passport validity can be found on the challenge.

### Tasks

1. Determine which passports are valid based on field presence
2. Determine which passports are valid based on field presence and content

### Solutions

##### 1 and 2: solution.rs

As task 2 criteria are a superset of task 1 criteria there is a single solution run in different modes.
```bash
rustc solution.rs
# Task 1
./solution input
# or
./solution.js

# Task 2
./solution input strict
# or
STRICT=1 ./solution.js
```

[1]: <https://adventofcode.com/2020/day/4> "Advent of Code day 4 challenge"
