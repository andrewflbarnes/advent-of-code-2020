
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OpCode {
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
pub struct Op {
    pub op: OpCode,
    pub operand: i32,
    pub executions: i32,
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

pub struct State {
    pub acc: i32,
    pub idx: usize,
    pub complete: bool,
}

impl State {
    pub fn new() -> State {
        State{
            acc: 0,
            idx: 0,
            complete: false,
        }
    }

    pub fn execute(&mut self, op: &mut Op) {
        match op.op {
            OpCode::Done => self.complete = true,
            _ => self.idx = op.execute(&mut self.acc, self.idx as i32) as usize
        }
    }
}

impl Op {
    pub fn execute(&mut self, acc: &mut i32, idx: i32) -> i32 {
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

pub fn parse_ops(raw: Vec<String>) -> Vec<Op> {
    let mut ops = vec![];

    raw.iter()
        .for_each(|line| {
            let op = line.parse::<Op>().unwrap();
            ops.push(op);
        });

    ops.push("done 0".parse::<Op>().unwrap());

    ops
}