# Challenge 6

The challenge can be found [here][1]

The `input` is a set of answer to questions. Each individuals "yes" answers are on a single line and
delimtied by a newline. Each group of people is delimited by a blank line.

### Tasks

1. Determine what the number of questions any person answered "yes" for each group and sum across all
groups
2. Determine what the number of questions all people answered "yes" for each group and sum across all
groups

### Solutions

##### 1 and 2

Due to similarity in processing a single solultion is provided:
```bash
rustc solution.rs
./solution input
# or
gcc -o solution.o solution.c
./solution.o
```

### Pain points

##### solution.c: array initialisation

I was mistakenly under the impression that newly allocated int arrays would have all values
initialised to 0. This appears to only be the case for static/global variables. There is some
syntactic sugar for initialising to zeroes:
```c
int arr[26] = {0};
```

##### solution.c: line buffer size

Note that the line buffer `line` is set to have a capacity of 28 characters. This is because
it requires
- Up to 26 characters for the full alphabet (assuming a person answers yes to all questions)
- 1 character for the newline
- 1 character for the null terminator

Initially I had this set to 27 (forgot about the newline char being included in the read) which
introduced a subtle and hard to diagnose bug. (Fortunately I wrote the C implementation after)
completing the challenge so I knew there was a bug and didn't delude myself into thinking AOC
was wrong!)

Any answer lines within a group appearing after an answer line which contained all letters were
considered a separate group resulting in the accumulated any and all counts being artifically
increased.

For example - consider the following `input` file with a single group
say we have input with a single group
```
abcdefghijklmnopqrstuvwxyz
abcdefghijklmnopqrstuvwxy
abcdefghijklmnopqrstuvwx
```

This should be treated as a single group with an "all" count of 24 and an "any" count of 26.
Instead the below happens (when the `line` variables is sized to 27):
- 26 chars + null terminator read from line 1 (27 chars)
- any and all counts set to 26
- newline character read from line 1 (misinterpretted end of group)
- accumulated any and all counts both set to 26
- 25 chars + newline + null terminator read from line 2 (27 chars)
- any and all counts set to 25
- 24 chars + newline + null terminator read from line 3 (26 chars)
- any and all counts set to 25 and 24 respectively
- newline character read from line 4 (end of group)
- accumulated any and all counts incremented to 51 and 50 respectively
- any count is off by +25 and all count is off by +26

I was finally able to diagnose the issue by emitting identical debug output from both the rust
and C implementations then diffing the output to find out where they diverged. I left these
debug messages in the source (commented out) for posterity.

[1]: <https://adventofcode.com/2020/day/6> "Advent of Code day 6 challenge"
