use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum OpCode {
    Acc,
    Jmp,
    Noop,
}

impl FromStr for OpCode {
    type Err = String;
    fn from_str(s: &str) -> Result<OpCode, Self::Err> {
        match s {
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            "nop" => Ok(OpCode::Noop),
            _ => Err(Self::Err::from(format!("Invalid opcode: {}", s)))
        }
    }
}

#[derive(Debug)]
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
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut ops: Vec<Op> = vec![];
    
    fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .for_each(|line| {
            let op = line.parse::<Op>().unwrap();
            ops.push(op);
        });
        
    // println!("{:?}", ops);

    let mut idx = 0;
    let mut acc = 0;
    loop {
        let op = &mut ops[idx];

        if op.executions > 0 {
            break;
        }

        idx = op.execute(&mut acc, idx as i32) as usize;
    }

    println!("Program begins looping at index {} with accumulator {}", idx, acc);
}