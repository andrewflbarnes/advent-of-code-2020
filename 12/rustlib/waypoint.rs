use super::operations::Operation;

#[derive(Debug)]
pub struct Waypoint {
    position: (i32, i32),
}

impl Waypoint {
    pub fn new() -> Waypoint {
        return Waypoint {
            position: (10, 1),
        }
    }

    pub fn execute(&mut self, operation: &Operation, position: (i32, i32)) -> (i32, i32) {
        match operation {
            Operation::North(d) => self.position.1 += d,
            Operation::South(d) => self.position.1 -= d,
            Operation::East(d) => self.position.0 += d,
            Operation::West(d) => self.position.0 -= d,
            Operation::Right(degrees) => {
                let times = degrees / 90;
                for _ in 0..times {
                    let (x, y) = self.position;
                    self.position.0 = y;
                    self.position.1 = -x;
                }
            },
            Operation::Left(degrees) => {
                let times = degrees / 90;
                for _ in 0..times {
                    let (x, y) = self.position;
                    self.position.0 = -y;
                    self.position.1 = x;
                }
            },
            Operation::Forward(d) => {
                let (vec_x, vec_y) = self.position;
                return (position.0 + (vec_x * d), position.1 + (vec_y * d))
            },
        }

        return position
    }
}