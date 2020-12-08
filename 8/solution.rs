use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum OpCode {
    Acc,
    Jmp,
    Noop,
    Done,
}

impl FromStr for OpCode {
    type Err = String;
    fn from_str(s: &str) -> Result<OpCode, Self::Err> {
        match s {
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            "nop" => Ok(OpCode::Noop),
            "done" => Ok(OpCode::Done),
            _ => Err(Self::Err::from(format!("Invalid opcode: {}", s)))
        }
    }
}

#[derive(Debug, Clone)]
struct Op {
    op: OpCode,
    operand: i32,
    executions: i32,
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Op, Self::Err> {
        let mut clauses = s.split(" ");

        Ok(Op{
            op: clauses.next().unwrap().parse().unwrap(),
            operand: clauses.next().unwrap().parse().unwrap(),
            executions: 0,
        })
    }
}

struct State {
    acc: i32,
    idx: usize,
    complete: bool,
}

impl State {
    fn new() -> State {
        State{
            acc: 0,
            idx: 0,
            complete: false,
        }
    }

    fn execute(&mut self, op: &mut Op) {
        match op.op {
            OpCode::Done => self.complete = true,
            _ => self.idx = op.execute(&mut self.acc, self.idx as i32) as usize
        }
    }
}

impl Op {
    fn execute(&mut self, acc: &mut i32, idx: i32) -> i32 {
        self.executions += 1;

        match self.op {
            OpCode::Acc => {
                *acc += self.operand;
                return idx + 1;
            },
            OpCode::Jmp => {
                return idx + self.operand;
            },
            OpCode::Noop => {
                return idx + 1;
            },
            OpCode::Done => {
                return idx;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut ref_ops: Vec<Op> = vec![];
    
    fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .for_each(|line| {
            let op = line.parse::<Op>().unwrap();
            ref_ops.push(op);
        });

    ref_ops.push("done 0".parse::<Op>().unwrap());
    let mut ops = ref_ops.clone();
        
    // println!("{:?}", ops);

    // task 1
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

    //task 2
    let mut mut_idx = 0;
    loop {
        state.idx = 0;
        state.acc = 0;
        state.complete = false;
        let mut mut_ops = ref_ops.clone();
        let desc;

        loop {
            let op = &mut mut_ops[mut_idx];
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
            let op = &mut mut_ops[state.idx];

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