use std::env;
use std::fs;

mod lib;
use lib::{
    Operation,
    State,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let ops: Vec<Operation> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| line.parse::<Operation>().unwrap())
        .collect();

    let mut state = State::new();
    ops.iter()
        .for_each(|op| state.execute(op));

    println!("Sum of memory values {}", state.mem_sum());

    let mut state = State::new();
    ops.iter()
        .for_each(|op| state.execute_2(op));

    println!("Sum of memory values {}", state.mem_sum());
}