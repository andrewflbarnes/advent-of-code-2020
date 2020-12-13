use std::str::FromStr;

#[derive(Debug)]
pub enum Heading {
    East,
    West,
    North,
    South,
}

impl Heading {
    pub fn rotate(&self, degrees: i32) -> Heading {
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

        match new_heading % 4 {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => panic!(format!("Invalid heading {} from initial {} ({:?}) offset {} ({})", new_heading, initial, self, offset, degrees)),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
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