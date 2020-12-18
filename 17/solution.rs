use std::env;
use std::fs;

mod lib3d;
mod lib4d;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let cycles = args[2].parse::<usize>().unwrap();
    // let cycles = 6;

    let input: Vec<String> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect();

        // Task 1
        let mut state = lib3d::State::new(input.clone(), cycles);
    
        for _ in 0..cycles {
            state.cycle();
        }
    
        // println!("{}", state);
        println!("Active after {} cycles in 3-space: {}", cycles, state.count_active());

        // Task 2
        let mut state = lib4d::State::new(input.clone(), cycles);
    
        for _ in 0..cycles {
            state.cycle();
        }
    
        // println!("{}", state);
        println!("Active after {} cycles in 4-space: {}", cycles, state.count_active());

}