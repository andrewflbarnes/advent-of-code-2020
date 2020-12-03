# Challenge 1

### Tasks

- 1: Find 2 values in `input` summing to 2020 and calculate their product
- 2: Find 3 values in `input` summing to 2020 and calculate their product

### Solutions

##### 1: solution.sh
Naive solution for first task. Runs as
```bash
./solution.sh
# or
rustc -o solution.o solution.rs
./solution.o
# or
gcc -o solution.o solution.c
./solution.o
```

##### 2: solution2.sh

General purpose solution for finding `n` values summing to `s` and calculating
their product. Generally run as
```bash
./solution2.sh <n> <s>
# i.e. for task 2
./solution2.sh 3 2020
# or (naive implementation)
rustc -o solution2.o solution2.rs
./solution2.o input 3 2020
```

There is also a naive C implemetation extending the `solution.c`
```
# or (naive implementation)
gcc -o solution2.o solution2.c
./solution2.o
```

Note: varying levels of debug output can be triggered by having the first
argument as one of `-v`, `-vv` or `-vvv`.
