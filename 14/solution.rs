use std::env;
use std::fs;

mod lib;
use lib::{
    MaskType,
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

    [MaskType::Value, MaskType::Address].iter()
        .for_each(|mask_type| {
            let mut state = State::new(*mask_type);
            ops.iter()
                .for_each(|op| state.execute(op));

            println!("Sum of memory values with {:?} masking {}", mask_type, state.mem_sum());
        });
}