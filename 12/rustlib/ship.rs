use super::operations::{
    Heading,
    Operation,
};

#[derive(Debug)]
pub struct Ship {
    heading: Heading,
    position: (i32, i32),
}

impl Ship {
    pub fn new() -> Ship {
        return Ship {
            heading: Heading::East,
            position: (0, 0),
        }
    }

    pub fn position(&self) -> (i32, i32) {
        return self.position.clone();
    }

    pub fn m_position(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    pub fn execute(&mut self, operation: &Operation) {
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