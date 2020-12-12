use std::env;
use std::fs;

mod rustlib;
use rustlib::{
    operations::Operation,
    ship::Ship,
    waypoint::Waypoint,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let operations: Vec<Operation> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| line.parse::<Operation>().unwrap())
        .collect();

    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    let mut waypoint_position = (0, 0);

    operations.iter().for_each(|op| {
        // print!("{:?} -> {:?} -> ", ship, op);
        ship.execute(op);
        // println!("{:?}", ship);

        // print!("{:?} {:?} -> {:?} -> ", waypoint, waypoint_position, op);
        waypoint_position = waypoint.execute(op, waypoint_position);
        // println!("{:?} {:?}", waypoint, waypoint_position);

    });

    println!("Task 1 final ship position {:?} - manhatten position {}", ship.position(), ship.m_position());
    println!("Task 2 final ship position {:?} - manhattan position {}", waypoint_position, waypoint_position.0.abs() + waypoint_position.1.abs());
}