use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let values = read_file(filename);

    let count = values.len();

    for i in 0..(count - 1) {
        let val1 = values[i];
        for j in (i + 1)..count {
            let val2 = values[j];
            if (val1 + val2) == 2020 {
                println!("Found {} and {} with product {}", val1, val2, val1 * val2);
                return;
            }
        }
    }
}

fn read_file(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect(&("Unable to read file ".to_owned() + filename))
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}