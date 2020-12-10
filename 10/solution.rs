use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let expected_longest_one_peat = 5;
    
    let mut data: Vec<i32> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| {
            let val = line.parse::<i32>().unwrap();
            return val;
        })
        .collect();

    data.sort();
    // Add the final device which is always 3 jolts higher
    data.push(data.last().unwrap() + 3);
    // println!("{:?}", data);

    let mut one_peat = 1;
    let mut longest_one_peat = 0;
    let mut combinations = 1u64;

    let out = data.iter()
        // (last value, count 1 jolt increments, count 3 jolt increments)
        .fold((&0i32, 0i32, 0i32), |mut acc, val| {
            match val - acc.0 {
                1 => {
                    acc.1 += 1;
                    one_peat += 1;
                },
                3 => {
                    acc.2 += 1;
                    if one_peat > longest_one_peat {
                        longest_one_peat = one_peat;
                    }
                    match one_peat {
                        1 => { /* No additional combinations possible */}
                        2 => { /* No additional combinations possible */}
                        // Worke the below out with pen and paper after doing a run to calculate longest_one_peat
                        3 => combinations *= 2,
                        4 => combinations *= 4,
                        5 => combinations *= 7,
                        _ => { /* Error at bottom */},
                    }
                    one_peat = 1;
                },
                _ => println!("Invalid joltage drop: {} {} -> {}", acc.0, val, val - acc.0),
            }

            acc.0 = val;

            return acc;
        });

    println!("Product of 1 jolt adapter increments ({}) and 3 jolt adapter increments ({}): {}", out.1, out.2, out.1 * out.2);
    if longest_one_peat != expected_longest_one_peat {
        println!("Max expected 1 repeats of up to {} in length but got {}, next value is lower bound", expected_longest_one_peat, one_peat);
    }
    println!("Number of combinations: {} (longest 1 jolt increment repetition: {})", combinations, longest_one_peat);

}