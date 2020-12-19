use std::env;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    OpenBrace,
    CloseBrace,
}

impl Op {
    fn apply(&self, acc: i64, val: i64) -> i64 {
        match self {
            Op::Add => acc + val,
            Op::Multiply => acc * val,
            Op::OpenBrace => panic!("Unsupported operation - apply {:?} to values {} and {}", self, acc, val),
            Op::CloseBrace => panic!("Unsupported operation - apply {:?} to values {} and {}", self, acc, val),
        }
    }
}

struct Expression {
    raw: String,
    precedence: Vec<Op>,
    val_stack: Vec<i64>,
    op_stack: Vec<Op>,
    sub_stack: Vec<(Vec<i64>, Vec<Op>)>,
}

impl Expression {
    fn new(raw: String, precedence: Vec<Op>) -> Expression {
        Expression {
            raw,
            precedence,
            val_stack: vec![],
            op_stack: vec![],
            sub_stack: vec![],
        }
    }

    fn process(&mut self) -> i64 {
        for ch in self.raw.clone().chars() {
            match ch {
                '(' => self.apply_op(Op::OpenBrace),
                ')' => self.apply_op(Op::CloseBrace),
                '+' => self.apply_op(Op::Add),
                '*' => self.apply_op(Op::Multiply),
                _ => {
                    let val = String::from(ch).parse::<i64>().unwrap();
                    self.apply_val(val);
                },
            }
        }

        self.process_current_stacks()
    }

    fn apply_val(&mut self, val: i64) {
        self.val_stack.push(val);
    }

    fn apply_op(&mut self, op: Op) {
        match op {
            Op::OpenBrace => {
                // println!("Push to stack {} {:?}", sub, op);
                self.sub_stack.push((self.val_stack.clone(), self.op_stack.clone()));
                self.val_stack = vec![];
                self.op_stack = vec![];
            },
            Op::CloseBrace => {
                let val = self.process_current_stacks();
                let (prev_vals, prev_ops) = self.sub_stack.pop().unwrap();
                // print!("Popped from stack {} {:?} with {} {:?} to ", prev_sub, prev_op, sub, op);
                self.val_stack = prev_vals;
                self.val_stack.push(val);
                self.op_stack = prev_ops;
            },
            _ => self.op_stack.push(op),
        }
    }

    fn process_current_stacks(&mut self) -> i64 {
        // Should be able to set whatever precedence is desired for any future ops here
        for op in self.precedence.iter() {
            let mut i = 0;
            while i < self.op_stack.len() {
                if self.op_stack[i] == *op {
                    self.val_stack[i] = op.apply(self.val_stack[i], self.val_stack[i + 1]);
                    self.val_stack.remove(i + 1);
                    self.op_stack.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        while self.op_stack.len() > 0 {
            self.val_stack[0] = self.op_stack[0].apply(self.val_stack[0], self.val_stack[1]);
            self.val_stack.remove(1);
            self.op_stack.remove(0);
        }
        
        self.val_stack[0]
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

    // task 1
    let sum = input.iter()
        .map(|line| {
            let mut expression = Expression::new(line.to_owned(), vec![]); 
            let res = expression.process();
            // println!("{} = {}", expression.raw, res);
            res
        })
        .sum::<i64>();

    println!("Sum of all expressions with no precedence: {}", sum);

    // task 2
    let sum = input.iter()
        .map(|line| {
            let mut expression = Expression::new(line.to_owned(), vec![Op::Add, Op::Multiply]); 
            let res = expression.process();
            // println!("{} = {}", expression.raw, res);
            res
        })
        .sum::<i64>();

    println!("Sum of all expressions with [add, multiply] precedence: {}", sum);
}