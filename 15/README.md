# Challenge 15

The challenge can be found [here][1]

This `input` is a set of starting numbers for a game. The game is played as below
- begin with a list of "starting" numbers
- consider the last number spoken aloud (or the last "starting" number if you are first to speak)
- if the number has appeared before give the difference between the positions of the last 2 occurrences
- if the number has not appeared before say 0

e.g. imagine we want to find the 12th number with a "starting" list of `1, 4, 10, 0, 1`. The next numbers are:
- `4` - the last number is `1` ,at the 5th position, which previously occurred at the 1st position
- `4` - the last number is `4`, at the 6th position, which previously occurred at the 2nd position
- `1` - the last number is `4`, at the 7th position, which previously occurred at the 6th position
- `3` - the last number is `1`, at the 8th position` which previously occurred at the 5th position
- `0` - the last number is `3`, which has not yet occurred
- `6` - the last number is `0`, at the 10th position, which previously occurred at the 4th position
- `0` - the last number is `6`, which has not yet occurred

`0` is the 12th number in this case (we include the "starting" numbers).

### Tasks

1. What is the 2020th value read aloud
2. What is the 30,000,000th value read aloud

### Solutions

##### 1 and 2

```bash
rustc solution.rs
./solution input
```

### notes

This was fairly simple in that the solution for task 2 just meant plugging in a bigger number. Of note is that
the second solve took *significantly* longer, perhaps as long as 30 seconds. Unfortunately haven't had the time
to check if this is an implementation issue or if there is a clever optimisation related more generally to this
class of problems.

[1]: <https://adventofcode.com/2020/day/15> "Advent of Code day 15 challenge"
