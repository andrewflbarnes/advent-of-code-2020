use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let levels = args[2].parse::<usize>().unwrap();
    let target = args[3].parse::<i32>().unwrap();

    let values = read_file(filename);

    let mut result: Vec<i32> = vec![];

    if resolve_level_target(&values, levels, target, 0, &mut result) {
        let mut product: i64 = 1;
        for val in result.clone() {
            product *= val as i64;
        }
        println!("Found {} values {:?} summing to {} with product {}", levels, result, target, product);
    } else {
        println!("Unable to find {} values summing to {}", levels, target);
    }
}

fn resolve_level_target(values: &Vec<i32>, level: usize, target: i32, start: usize, mut result: &mut Vec<i32>) -> bool {
    let max = values.len();

    for i in start..(max - level + 1) {
        let value = values[i];
        if level == 1 {
            if value == target {
                result.push(value);
                // println!("Found {} -> {:?}", target, result);
                return true;
            }
        } else if resolve_level_target(&values, level - 1, target - value, i + 1, &mut result) {
            result.push(value);
            // println!("Found {} -> {:?}", target, result);
            return true;
        }
    }

    // println!("Level {} loop exhausted for {}", level, (start - 1));

    false
}

fn read_file(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect(&("Unable to read file ".to_owned() + filename))
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}