use std::env;
use std::fs;

mod operations;
use operations::{
    State,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let data = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect();

    let mut ops = operations::parse_ops(data);
    // println!("{:?}", ref_ops);

    let mut state = State::new();
    loop {
        let op = &mut ops[state.idx];

        if op.executions > 0 {
            println!("Program begins looping at index {} with accumulator {}", state.idx, state.acc);
            break;
        }

        state.execute(op);

        if state.complete {
            println!("UNEXPECTED: Program completed executing at index {} with accumulator {}", state.idx, state.acc);
            break;
        }
    }
}