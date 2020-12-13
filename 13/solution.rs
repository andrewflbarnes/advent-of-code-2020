use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let lines: Vec<String> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect();

    let departure = lines[0].parse::<i32>().unwrap();
    let buses: Vec<i32> = lines[1]
        .split(",")
        .filter(|bus| bus != &"x")
        .map(|bus| bus.parse::<i32>().unwrap())
        .collect();
    
        println!("{} - {:?}", departure, buses);

    let earliest: (i32, i32) = buses.iter()
        .map(|bus| {
            let wait_time = bus - (departure % bus);
            (bus, wait_time)
        })
        .fold((0, 999999999), |acc, val| {
            if val.1 < acc.1 {
                (*val.0, val.1)
            } else {
                acc
            }
        });

    println!("{:?} : {}", earliest, earliest.0 * earliest.1);
}