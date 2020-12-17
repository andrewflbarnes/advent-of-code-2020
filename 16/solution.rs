use std::env;
use std::fs;

mod lib;
use lib::{
    Criterion,
    Ticket,
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut is_criteria = true;
    let mut is_my_ticket = false;
    let mut skip = false;
    let mut criteria: Vec<Criterion> = vec![];
    let mut my_ticket: Ticket = Ticket::null();
    let mut tickets: Vec<Ticket> = vec![];

    fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .for_each(|line| {
            if line.len() == 0 {
                if is_criteria {
                    is_my_ticket = true;
                    is_criteria = false;
                } else if is_my_ticket {
                    is_my_ticket = false;
                }
                skip = true;
            } else if skip {
                skip = false;
            } else if is_criteria {
                criteria.push(line.parse::<Criterion>().unwrap());
            } else if is_my_ticket {
                my_ticket = line.parse::<Ticket>().unwrap();
                tickets.push(line.parse::<Ticket>().unwrap());
            } else {
                tickets.push(line.parse::<Ticket>().unwrap());
            }
        });

    // Task 1
    let error_rate = tickets.iter()
    .map(|t| t.is_valid(&criteria).1)
    .fold(0u32, |acc, val| acc + val);
    
    println!("Error rate: {}", error_rate);
    
    // Task 2
    let valid_tickets: Vec<&Ticket> = tickets.iter()
    .filter(|t| t.is_valid(&criteria).0)
    .collect();
    
    println!("Valid tickets: {} of {}", valid_tickets.len(), tickets.len());
    
    let mapping = lib::determine_fields(&valid_tickets, &criteria);
    let departure_product: u64 = mapping.iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| &my_ticket.fields[*idx])
        .map(|u_32| *u_32 as u64)
        .fold(1u64, |acc, val| acc * val);
    
    println!("Departure product: {}", departure_product);

    Ok(())
}