use std::env;
use std::fs;
use std::io::{
    BufReader,
    BufRead,
};

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

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = fs::File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut valid = 0;

    loop {
        match buf.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                if Password::parse(&line).is_valid() {
                    valid += 1;
                }

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    println!("{} passwords valid", valid);

    Ok(())
}