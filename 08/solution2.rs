use std::env;
use std::fs;

mod operations;
use operations::{
    OpCode,
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

    // keep a reference to the original ops (with initialised state) as they will be reused mutliple times
    let ref_ops = operations::parse_ops(data);
    // println!("{:?}", ref_ops);

    let mut state = State::new();
    let mut mut_idx = 0;

    loop {
        state.idx = 0;
        state.acc = 0;
        state.complete = false;
        let mut ops = ref_ops.clone();
        let desc;

        loop {
            let op = &mut ops[mut_idx];
            mut_idx += 1;

            match op.op {
                OpCode::Jmp => {
                    desc = format!("instruction {} jmp->noop", mut_idx - 1);
                    op.op = OpCode::Noop;
                    break;
                },
                OpCode::Noop => {
                    desc = format!("instruction {} noop->jmp", mut_idx - 1);
                    op.op = OpCode::Jmp;
                    break;
                },
                _ => {},
            }
        }

        loop {
            let op = &mut ops[state.idx];

            if op.executions > 0 {
                // println!("FAILURE: Program begins looping at index {} with accumulator {} (changed {})", state.idx, state.acc, desc);
                break;
            }

            state.execute(op);

            if state.complete {
                break;
            }
        }

        if state.complete {
            println!("Program completed executing at index {} with accumulator {} (changed {})", state.idx, state.acc, desc);
            break;
        }
    }
}