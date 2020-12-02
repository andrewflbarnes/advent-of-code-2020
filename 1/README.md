# Challenge 1

### Tasks

- 1: Find 2 values in `input` summing to 2020 and calculate their product
- 2: Find 3 values in `input` summing to 2020 and calculate their product

### Solutions

##### 1: solution.sh
Naive solution for first task. Runs as
```bash
./solution.sh
```

##### 2: solution2.sh

General purpose solution for finding `n` values summing to `s` and calculating
their product. Generally run as
```bash
./solution2.sh <n> <s>
# i.e. for task 2
./solution2.sh 3 2020
```

Note: varying levels of debug output can be triggered by having the first
argument as one of `-v`, `-vv` or `-vvv`.
