use std::env;
use std::fs;

#[derive(Debug)]
struct Password<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

impl <'a>Password<'a> {
    pub fn parse(line: &'a str) -> Password {
        let atoms: Vec<&str> = line.split(" ").collect();
        let constraint: Vec<&str> = atoms[0].split("-").collect();
        let min = constraint[0].parse::<usize>().unwrap();
        let max = constraint[1].parse::<usize>().unwrap();
        let letter = atoms[1].chars().nth(0).unwrap();
        let password = atoms[2];
        Password {
            min,
            max,
            letter,
            password,
        }
    }

    pub fn is_valid(&self) -> bool {
        let matched = self.password
            .chars()
            .filter(|c| c == &self.letter)
            .count();
        matched >= self.min && matched <= self.max
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let values = read_file(filename);

    let mut valid = 0;
    for v in values {
        let password = Password::parse(&v);
        // println!("CHECK   : {:?}", password);
        if password.is_valid() {
            // println!("VALID   : {:?}", password);
            valid += 1;
        } else {
            // println!("INVALID : {:?}", password);
        }
    }
    println!("{} passwords valid", valid);
}

fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect(&("Unable to read file ".to_owned() + filename))
        .lines()
        .map(String::from)
        .collect()
}