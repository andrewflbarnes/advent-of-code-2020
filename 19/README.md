# Challenge 19

The challenge can be found [here][1]

Input is a set of pattern matching rules followed by strings to match against. Matching rules have an ID
and are constructed as below:
- if `"<letter>"` matches that letter
- if consisting of space separated numbers then must match of those rules in order
- when a `|` (pipe) is seen then may match the expression either side

e.g. for
```
0: 1 2 | 3 2
1: 5 4 | 4 5
2: 4 | 5
3: 4 5
4: "a"
5: "b"
```
We have equivalent regex of
```
0: ((ba|ab)(a|b)|b(a|b))
1: (ba|ab)
2: (a|b)
3: ab
4: a
5: b
```

For task 2 the below rules are adjusted: completely replace rules 8: 42 and 11: 42 31 with the following:
```
8: 42 | 42 8
11: 42 31 | 42 11 31
```
Note that this results in recursive (potentially infinite) loops.

### Tasks

1. How many strings match rule `0`
2. How many strings match rule `0` with the adjusted rules

### Solutions

##### 1 and 2

```bash
cargo run input 0
```

### notes
Used `Cargo` to ease importing the `regex` crate.

[1]: <https://adventofcode.com/2020/day/19> "Advent of Code day 19 challenge"
