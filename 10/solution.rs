use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let mut data: Vec<i32> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| {
            let val = line.parse::<i32>().unwrap();
            println!("{}", val);
            return val;
        })
        .collect();

    data.sort();
    println!("{:?}", data);

    // Start with a count of 1 for 3 jolt increases to include the "device" being connected
    let out = data.iter()
        .fold((&0i32, 0i32, 1i32), |mut acc, val| {
            match val - acc.0 {
                1 => acc.1 += 1,
                3 => acc.2 += 1,
                _ => println!("Invalid voltage drop: {} {} -> {}", acc.0, val, val - acc.0),
            }

            acc.0 = val;

            return acc;
        });

    println!("{}", out.1 * out.2);

}