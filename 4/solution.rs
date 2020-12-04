use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::{
    BufReader,
    BufRead,
};

static FIELDS: &'static [&'static str] = &[ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
static HEX: &'static [char] = &[ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', ];
static EYE_COLORS: &'static [&'static str] = &[ "amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl <'a>Passport {
    pub fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }

    pub fn parse(&mut self, line: &'a str) {
        for field in line.split(" ") {
            let atoms: Vec<&str> = field.split(":").collect();
            self.fields.insert(String::from(atoms[0]), String::from(atoms[1]));
        }
    }

    pub fn reset(&mut self) {
        self.fields.clear();
    }

    pub fn is_valid(&self, strict: bool) -> bool {
        for field in FIELDS.into_iter() {
            // whyyyyy is this required???
            // error[E0277]: the trait bound `std::string::String: std::borrow::Borrow<&str>` is not satisfied
            //   --> solution.rs:39:29
            //   |
            // 39 |             if !self.fields.contains_key(field) {
            //   |                             ^^^^^^^^^^^^ the trait `std::borrow::Borrow<&str>` is not implemented for `std::string::String`
            //   |
            //   = help: the following implementations were found:
                        // <std::string::String as std::borrow::Borrow<str>>
            let f: &str = field;
            match self.fields.get(f) {
                None => return false,
                Some(v) => {
                    if strict {
                        if !self.validate_field(f, v) {
                            return false;
                        }
                    }
                },
            } 
        }

        true
    }

    fn validate_field(&self, f: &str, v: &str) -> bool {
        match f {
            "byr" => {
                let y = String::from(v).parse::<i32>().unwrap();
                if y < 1920 || y > 2002 {
                    return false;
                }
            },
            "iyr" => {
                let y = String::from(v).parse::<i32>().unwrap();
                if y < 2010 || y > 2020 {
                    return false;
                }
            },
            "eyr" => {
                let y = String::from(v).parse::<i32>().unwrap();
                if y < 2020 || y > 2030 {
                    return false;
                }
            },
            "hgt" => {
                let h = String::from(&v[..(v.len() - 2)]).parse::<i32>().unwrap();
                let u = &v[(v.len() - 2)..];
                match u {
                    "cm" => {
                        if h < 150 || h > 193 {
                            return false;
                        }
                    },
                    "in" => {
                        if h < 59 || h > 76 {
                            return false;
                        }
                    },
                    _ => return false,
                }
            },
            "hcl" => {
                if &v[0..1] != "#" || v.len() != 7 {
                    return false
                }

                if !&v[1..].chars().all(|c| HEX.contains(&c)) {
                    return false;
                }
            },
            "ecl" => {
                if !EYE_COLORS.contains(&v) {
                    return false;
                }
            },
            "pid" => {
                println!("{} {}", v, v.len());
                if v.len() != 9 || !v.chars().all(char::is_numeric) {
                    return false;
                }
            },
            _ => {},
        }

        true
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let strict: bool;
    if args.len() > 2 {
        strict = args[2] == "strict";
    } else {
        strict = false;
    }
    match strict {
        true  => println!("Strict processing enabled"),
        false => println!("Strict processing disabled"),
    }

    let file = fs::File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut valid = 0;
    let mut passport = Passport::new();

    loop {
        match buf.read_line(&mut line) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    // EOF
                    break;
                }

                // remove newline
                line.pop();
                if !line.is_empty() {
                    passport.parse(&line.clone());
                } else {
                    if passport.is_valid(strict) {
                        valid += 1;
                    }
                    passport.reset();
                }

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    println!("Valid passports: {}", valid);

    Ok(())
}