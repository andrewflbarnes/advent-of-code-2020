use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Heading {
    East,
    West,
    North,
    South,
}

impl Heading {
    fn rotate(&self, degrees: i32) -> Heading {
        let offset = degrees / 90;

        let initial = match self {
            Heading::North => 0,
            Heading::East => 1,
            Heading::South => 2,
            Heading::West => 3,
        };

        let mut new_heading = initial + offset;
        if new_heading < 0 {
            new_heading += 4;
        }

        let result = match new_heading % 4 {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => panic!(format!("Invalid heading {} from initial {} ({:?}) offset {} ({})", new_heading, initial, self, offset, degrees)),
        };

        result
    }
}

#[derive(Debug)]
enum Operation {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Right(i32),
    Left(i32),
    Forward(i32),
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        let val = s[1..].parse::<i32>().unwrap();
        match &s[0..1] {
            "N" => return Ok(Operation::North(val)),
            "S" => return Ok(Operation::South(val)),
            "E" => return Ok(Operation::East(val)),
            "W" => return Ok(Operation::West(val)),
            "R" => return Ok(Operation::Right(val)),
            "L" => return Ok(Operation::Left(val)),
            "F" => return Ok(Operation::Forward(val)),
            _ => Err(Self::Err::from(&format!("Unrecognised operation {}", s))),
        }
    }
}

#[derive(Debug)]
struct Ship {
    heading: Heading,
    position: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        return Ship {
            heading: Heading::East,
            position: (0, 0),
        }
    }

    fn m_position(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn execute(&mut self, operation: &Operation) {
        match operation {
            Operation::North(d) => self.position.1 += d,
            Operation::South(d) => self.position.1 -= d,
            Operation::East(d) => self.position.0 += d,
            Operation::West(d) => self.position.0 -= d,
            Operation::Right(degrees) => self.heading = self.heading.rotate(*degrees),
            Operation::Left(degrees) => self.heading = self.heading.rotate(-degrees),
            Operation::Forward(d) => {
                match self.heading {
                    Heading::North => self.position.1 += d,
                    Heading::South => self.position.1 -= d,
                    Heading::East => self.position.0 += d,
                    Heading::West => self.position.0 -= d,
                }
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let operations: Vec<Operation> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| line.parse::<Operation>().unwrap())
        .collect();

    let mut ship = Ship::new();

    operations.iter().for_each(|op| {
        print!("{:?} -> {:?} -> ", ship, op);
        ship.execute(op);
        println!("{:?}", ship);
    });

    println!("{:?}", ship);
    println!("{}", ship.m_position());
}