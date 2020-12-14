
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum MaskType {
    Address,
    Value,
}

#[derive(Debug)]
pub struct Mask {
    floats: Vec<usize>,
    zeroes: u64,
    ones: u64,
}

impl Mask {
    fn new() -> Mask {
        Mask {
            floats: vec![],
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
            } else if ch == "X" {
                mask.floats.push(len - i - 1);
            }
        }
        // println!("Parse mask:\nmm{}\n{:#038b}\n{:#038b}", s, mask.zeroes, mask.ones);
        // println!("Floats: {:?}", mask.floats);
        Ok(mask)
    }
}

#[derive(Debug)]
pub struct Mem {
    addr: usize,
    val: u64,
}

impl FromStr for Mem {
    type Err = String;
    fn from_str(s: &str) -> Result<Mem, Self::Err> {
        let mut atoms = s.split(" = ");
        let addr_str = atoms.next().unwrap();
        let range = (addr_str.find("[").unwrap() + 1)..addr_str.find("]").unwrap();
        let addr = addr_str[range].parse::<usize>().unwrap();
        let val = atoms.next().unwrap().parse::<u64>().unwrap();

        Ok(Mem{
            addr,
            val,
        })
    }
}

#[derive(Debug)]
pub enum Operation {
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
pub struct State {
    mem: HashMap<usize, u64>,
    mask: Mask,
    mask_type: MaskType,
}

impl State {
    pub fn new(mask_type: MaskType) -> State {
        State {
            mem: HashMap::new(),
            mask: Mask::new(),
            mask_type,
        }
    }

    pub fn execute(&mut self, op: &Operation) {
        match op {
            Operation::MaskUpdate(mask) => {
                self.mask.zeroes = mask.zeroes;
                self.mask.ones = mask.ones;
                self.mask.floats = mask.floats.clone();
                // println!("Mask updated to\nzeroes: {:#038b}\nones:   {:#038b}", mask.zeroes, mask.ones);
            },
            Operation::MemSet(mem) => {
                match self.mask_type {
                    MaskType::Value => {
                        let adjusted_val = mem.val & self.mask.zeroes | self.mask.ones;
                        // println!("Saving {} -> {} at index {}", mem.val, adjusted_val, mem.addr);
                        self.mem.insert(mem.addr, adjusted_val);
                    },
                    MaskType::Address => {
                        let init_addr = mem.addr | self.mask.ones as usize;
                        let floats = &mut self.mask.floats.clone();
                        // println!("Initial address {} with floats: {:?}", init_addr, floats);
                        self.write_floats(floats, init_addr, mem.val);
                    },
                }
            },
        }
    }

    fn write_floats(&mut self, idxs: &mut Vec<usize>, addr: usize, val: u64) {
        if idxs.len() == 0 {
            // println!("Saving {} at index {}", val, addr);
            self.mem.insert(addr, val);
        } else {
            let idx = idxs.pop().unwrap();

            let one_mask = 1 << idx;
            self.write_floats(idxs, addr | one_mask, val);

            let zero_mask = 0xFFFFFFFFF - (1 << idx);
            self.write_floats(idxs, addr & zero_mask, val);

            idxs.push(idx);
        }
    }

    pub fn mem_sum(&self) -> u64 {
        self.mem.iter()
            .fold(0u64, |acc, (_ ,val)| acc + val)
    }
}
