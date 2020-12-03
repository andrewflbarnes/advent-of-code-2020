use std::env;
use std::fs;

#[derive(Debug)]
struct Password<'a> {
    positions: (usize, usize),
    letter: char,
    password: &'a str,
}

impl <'a>Password<'a> {
    pub fn parse(line: &'a str) -> Password {
        let atoms: Vec<&str> = line.split(" ").collect();
        let constraint: Vec<&str> = atoms[0].split("-").collect();
        let positions = (
            constraint[0].parse::<usize>().unwrap() - 1,
            constraint[1].parse::<usize>().unwrap() - 1,
        );
        let letter = atoms[1].chars().nth(0).unwrap();
        let password = atoms[2];
        Password {
            positions,
            letter,
            password,
        }
    }

    pub fn is_valid(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        (chars[self.positions.0] == self.letter) ^
            (chars[self.positions.1] == self.letter)
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
    println!("{} password valid", valid);
}

fn read_file(filename: &str) -> Vec<String> {

    let contents = fs::read_to_string(filename)
        .expect(&("Unable to read file ".to_owned() + filename));

    let lines = contents.split("\n");

    let mut values: Vec<String> = vec![];

    for line in lines {
        if !line.is_empty() {
            values.push(String::from(line));
        }
    }

    values
}