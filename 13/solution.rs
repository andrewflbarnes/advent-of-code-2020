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

    let departure = lines[0].parse::<u64>().unwrap();
    let buses: Vec<(usize, u64)> = lines[1]
        .split(",")
        .enumerate()
        .filter(|(_, bus)| bus != &"x")
        .map(|(i, bus)| (i, bus.parse::<u64>().unwrap()))
        .collect();

    // task 1
    let (earliest_bus, earliest_wait) = buses.iter()
        .map(|(_, bus)| {
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
    
    println!("Earliest bus {} after {} minutes with product {}", earliest_bus, earliest_wait, earliest_bus * earliest_wait);

    // task 2
    let (_, first) = buses.first().unwrap();

    // Calculate the modular value per prime bus (excluding first bus)
    let buses: Vec<(u64, u64)> = buses[1..].iter()
        .map(|(bus_offset, bus_time)| {
            let mut modular = 0;
            loop {
                modular += 1;
                if ((first * modular) + *bus_offset as u64) % bus_time == 0 {
                    return (*bus_time, modular);
                }
            }
        })
        .collect();

    // println!("Modular values: {:?}", buses);
    
    // Calculate the minimum timestamp required
    let (total_mod, _) = buses.iter()
        .fold((0u64, 0u64), |(mut total_mod, increment), (bus_time, bus_mod)| {
            if total_mod == 0 {
                return (*bus_mod, *bus_time);
            }

            loop {
                total_mod += increment;
                if total_mod % bus_time == *bus_mod {
                    break;
                }
            }

            (total_mod, increment * bus_time)
        });

    println!("Earliest timestamp where all buses arrive at their offset: {}", first * total_mod);
}