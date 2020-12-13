# Challenge 12

The challenge can be found [here][1]

Challenge is a combination of modular arithmetic and prime number processing.

This `input` is two lines
- a timestamp (task 1 only)
- a list of buses where
    - if a number this is the bus ID and also how often it runs in minutes
    - if `x` then ignore
    - the position in the list is the offset in minutes for task 2

### Tasks

1. What is the shortest amount of time to wait for a bus and what is the product of the bus ID and wait time
2. What is the first occurring timestamp such that each bus arrives `n` minutes after that timestamp where `n`
is the offset in the bus listings

### Solutions

##### 1 and 2

```bash
rustc solution.rs
./solution input
```

### Notes

Task 2 was a bit of nightmare. A naive solution doesn't really work here because of the sheer amount of numbers
to check and calculations to perform. i.e. in a naive solution we
- start at 0 minutes
- increments by 19 minutes each time (the first bus which arrives at offset 0)
- check if each other bus will arrive `n` minutes after this
- if so we are done, if not repeat from the increment step

Given the answer is ~15 digits long we have approx 50 trillion values to check...

The below assumes knowledge of basic [modular arithmetic][2].

Instead we do the below
- find the first bus and it's periodicity (how often it runs). Any solution is eactly divisible by this number i,e `19n`
- we need a fast way to calculate `n`
- for each other bus calculate minimum `m` where `period<first bus> * m<bus> + offset<bus> = 0 (mod period<bus>)`. e.g.
    - for a first bus with periodicity `3` and three more buses with periodicity `5`, `7` and `11`, offset `1`, `2` and `4` minutes respectively:
        - the 5-period bus gives `m = 3` (`3m + 1 (mod 5)`)
        - the 7-period bus gives `m = 4` (`3m + 2 (mod 7)`)
        - the 11-period bus gives `m = 6` (`3m + 4 = (mod 11)`)
- Excluding `bus0` we know we have a minimum `n` of `m<bus1>` e.g. from above
    - `m<bus1>` is `3` and firt bus periodicity is `3`
    - `3 * 3 = 9`
    - `bus5` will arrive 1 minute after this time (`10` minutes as `10 % 5 == 0`)
- We now increment `n` (`3`) by `period<bus1>` (`5`) until `n % period<bus2> == m<bus2>` i.e. `n % 7 == 4`
    - Incrementing by `5` ensures that we always have `n = 3 (mod 5)`
- At the point `n % 7 == 4` (`18`) we fulfil both
    - `n = 3 (mod 5)`
    - `n = 4 (mod 7)`
- We now increment `n` (`18`) by `period<bus1> * period<bus2>` (`35`) until `n % period<bus3> == m<bus3>` i.e. `n % 7 == 4`
    - Incrementing by `35` ensures that we always have `n = 3 (mod 5)` and `n = 4 (mod 7)`
- At the point `n % 7 == 4` (`193`) we fulfil all of
    - `n = 3 (mod 5)`
    - `n = 4 (mod 7)`
    - `n = 6 (mod 11)`
- **If there are more buses then continue this pattern until they have all been used**
- Once this final `n` figure has been calculated mutliply it by the first bus periodicity e.g.
    - With `n = 193` from above and the first bus periodicty being `3` we get `579`
    - The first `5` bus to arrive after this is at `580` (`1` minute later as `580 % 5 == 0`)
    - The first `7` bus to arrive after this is at `581` (`2` minutes later as `581 % 7 == 0`)
    - The first `11` bus to arrive after this is at `584` (`4` minutes later as `584 % 11 == 0`)



[1]: <https://adventofcode.com/2020/day/13> "Advent of Code day 13 challenge"
[2]: <https://en.wikipedia.org/wiki/Modular_arithmetic> "Wikipedia: Modular Arithmetic"
