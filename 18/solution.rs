use std::env;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Op {
    Noop,
    Add,
    Multiply,
    OpenBrace,
    CloseBrace,
}

impl Op {
    fn apply(&self, acc: i64, val: i64) -> i64 {
        match self {
            Op::Noop => val,
            Op::Add => acc + val,
            Op::Multiply => acc * val,
            Op::OpenBrace => panic!("Unsupported operation - apply {:?} to values {} and {}", self, acc, val),
            Op::CloseBrace => panic!("Unsupported operation - apply {:?} to values {} and {}", self, acc, val),
        }
    }
}

struct Expression {
    stack: Vec<(i64, Op)>, 
    sub: i64,
    op: Op,
}

impl Expression {
    fn new() -> Expression {
        Expression {
            stack: vec![],
            sub: 0,
            op: Op::Noop,
        }
    }

    fn apply_val(&mut self, val: i64) {
        self.sub = self.op.apply(self.sub, val);
        self.op = Op::Noop;
    }

    fn apply_op(&mut self, op: Op) {
        match op {
            Op::OpenBrace => {
                // println!("Push to stack {} {:?}", sub, op);
                self.stack.push((self.sub, self.op));
                self.sub = 0;
                self.op = Op::Noop;
            },
            Op::CloseBrace => {
                let (prev_sub, prev_op) = self.stack.pop().unwrap();
                // print!("Popped from stack {} {:?} with {} {:?} to ", prev_sub, prev_op, sub, op);
                self.sub = prev_op.apply(prev_sub, self.sub);
                // println!("{}", sub);
                self.op = Op::Noop;
            },
            _ => self.op = op,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input: Vec<String> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .map(|s| s.replace(" ", ""))
        .collect();
        
    let sum = input.iter()
        .map(|line| {
            let mut expression = Expression::new(); 
            // println!("{}", line);
            for ch in line.chars() {
                match ch {
                    '(' => expression.apply_op(Op::OpenBrace),
                    ')' => expression.apply_op(Op::CloseBrace),
                    '+' => expression.apply_op(Op::Add),
                    '*' => expression.apply_op(Op::Multiply),
                    _ => {
                        let val = String::from(ch).parse::<i64>().unwrap();
                        expression.apply_val(val);
                    },
                }
            }
            // println!("{} = {}", line, sub);

            expression.sub
        })
        .sum::<i64>();


    println!("Sum of all expressions: {}", sum);
}