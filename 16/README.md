# Challenge 16

The challenge can be found [here][1]

Input is broken down into 3 sections
- a list of criteria for fields on a ticket
- the fields of your ticket
- the fields of nearby tickets

Other notes
- not all nearby tickets are valid
- tickets are invalid if they have at least one field which can't be matched to any criteria
- for all tickets fields appear in the same order

### Tasks

1. Determine which tickets are invalid and calculate the sum of all invalid fields
2. Determine which values correspond to which fields - for your ticket calculate the product of the
`departure*` fields

### Solutions

##### 1 and 2

```bash
rustc solution.rs
./solution input
```

### Notes

Rust was really working against me here - part of my solution used below form but this just wouldn't
compile (`fs.retain`) despite lots of tinkering:
```rust
for t in tickets {
    for (c, fs) in &mapping {
        let mut valid = vec![];
        for f in fs {
            if !t.is_field_valid(c, *f) {
                fs.retain(|x| x != f);
            } else {
                valid.push(*f);
            }
        }
    }
}
```

The solution (workaround) I eventually went with was just rebuilding the `mapping` `HashMap` on the fly
with a new `Vec` for the `fs` value - far from ideal.


[1]: <https://adventofcode.com/2020/day/16> "Advent of Code day 16 challenge"
