use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Mask {
    zeroes: u64,
    ones: u64,
}

impl Mask {
    fn new() -> Mask {
        Mask {
            zeroes: 0xFFFFFFFFFu64,
            ones: 0x0u64,
        }
    }
}

impl FromStr for Mask {
    type Err = String;
    fn from_str(s: &str) -> Result<Mask, Self::Err> {
        let mut mask = Mask::new();
        let len = s.len();
        for i in 0..len {
            let idx = len - i - 1;
            let ch = &s[i..i+1];
            if ch == "1" {
                mask.ones += 1 << idx;
            } else if ch == "0" {
                mask.zeroes -= 1 << idx;
            }
        }
        // println!("Parse mask:\nmm{}\n{:#038b}\n{:#038b}\n", s, mask.zeroes, mask.ones);

        Ok(mask)
    }
}

#[derive(Debug)]
struct Mem {
    idx: usize,
    val: u64,
}

impl FromStr for Mem {
    type Err = String;
    fn from_str(s: &str) -> Result<Mem, Self::Err> {
        let mut atoms = s.split(" = ");
        let idx_str = atoms.next().unwrap();
        let range = (idx_str.find("[").unwrap() + 1)..idx_str.find("]").unwrap();
        let idx = idx_str[range].parse::<usize>().unwrap();
        let val = atoms.next().unwrap().parse::<u64>().unwrap();

        Ok(Mem{
            idx,
            val,
        })
    }
}

#[derive(Debug)]
enum Operation {
    MaskUpdate(Mask),
    MemSet(Mem),
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        let mut atoms = s.split(" = ");
        let op_type = atoms.next().unwrap();
        let op_instruction = atoms.next().unwrap();
        match op_type {
            "mask" => Ok(Operation::MaskUpdate(op_instruction.parse::<Mask>().unwrap())),
            _ => Ok(Operation::MemSet(s.parse::<Mem>().unwrap())), // assume mem[i]
        }
    }
}

#[derive(Debug)]
struct State {
    mem: HashMap<usize, u64>,
    mask: Mask,
}

impl State {
    fn new() -> State {
        State {
            mem: HashMap::new(),
            mask: Mask::new(),
        }
    }

    fn execute(&mut self, op: &Operation) {
        match op {
            Operation::MaskUpdate(mask) => {
                self.mask.zeroes = mask.zeroes;
                self.mask.ones = mask.ones;
                // println!("Mask updated to\nzeroes: {:#038b}\nones:   {:#038b}", mask.zeroes, mask.ones);
            },
            Operation::MemSet(mem) => {
                let adjusted_val = mem.val & self.mask.zeroes | self.mask.ones;
                // println!("Memset v={}\nv>{:#038b}\n0>{:#038b}\n1>{:#038b}\n->{:#038b}\n", mem.val, mem.val, self.mask.zeroes, self.mask.ones, adjusted_val);
                // println!("Saving {} -> {} at index {}", mem.val, adjusted_val, mem.idx);
                self.mem.insert(mem.idx, adjusted_val);
            },
        }
    }

    fn mem_sum(&self) -> u64 {
        self.mem.iter()
            .fold(0u64, |acc, (_ ,val)| acc + val)
    }
}



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
}