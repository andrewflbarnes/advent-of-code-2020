use std::env;
use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Criterion {
    name: String,
    range: ((u32, u32), (u32, u32)),
}

impl FromStr for Criterion {
    type Err = String;
    fn from_str(s: &str) -> Result<Criterion, Self::Err> {
        let mut atoms = s.split(": ");
        let name = String::from(atoms.next().unwrap());

        let mut ranges = atoms.next().unwrap().split(" or ");
        let mut range_1_atoms = ranges.next().unwrap().split("-");
        let range_1 = (
            range_1_atoms.next().unwrap().parse::<u32>().unwrap(),
            range_1_atoms.next().unwrap().parse::<u32>().unwrap(),
        );
        let mut range_2_atoms = ranges.next().unwrap().split("-");
        let range_2 = (
            range_2_atoms.next().unwrap().parse::<u32>().unwrap(),
            range_2_atoms.next().unwrap().parse::<u32>().unwrap(),
        );

        Ok(Criterion {
            name,
            range: (range_1, range_2),
        })
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>,
}

impl Ticket {
    fn null() -> Ticket {
        Ticket{
            fields: vec![],
        }
    }

    fn is_valid(&self, criteria: &Vec<Criterion>) -> (bool, u32) {
        let mut rate = 0;
        let mut found = false;
        let mut valid = true;
        for f in &self.fields {
            for c in criteria {
                let (r0, r1) = c.range;
                if (f >= &r0.0 && f <= &r0.1) || (f >= &r1.0 && f <= &r1.1) {
                found = true;
                    break;
                }
            }
            if !found {
                rate += f;
                valid = false;
            } else {
                found = false;
            }
        }
        (valid, rate)
    }

    fn is_field_valid(&self, criterion: &Criterion, field: usize) -> bool {
        let f = self.fields.get(field).unwrap();
        let (r0, r1) = criterion.range;
        (f >= &r0.0 && f <= &r0.1) || (f >= &r1.0 && f <= &r1.1)
    }
}

impl FromStr for Ticket {
    type Err = String;
    fn from_str(s: &str) -> Result<Ticket, Self::Err> {
        let vals = s.split(",");
        let mut fields = vec![];
        for val in vals {
            fields.push(val.parse::<u32>().unwrap());
        }

        Ok(Ticket{
            fields,
        })
    }
}

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
    
    let mapping = determine_fields(&valid_tickets, &criteria);
    let departure_product: u64 = mapping.iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| &my_ticket.fields[*idx])
        .map(|u_32| *u_32 as u64)
        .fold(1u64, |acc, val| acc * val);
    
    println!("Departure product: {}", departure_product);

    Ok(())
}

fn determine_fields(tickets: &Vec<&Ticket>, criteria: &Vec<Criterion>) -> HashMap<String, usize> {
    let mut mapping:HashMap<&Criterion, Vec<usize>> = HashMap::new();

    let field_count = tickets[0].fields.len();
    for c in criteria {
        let mut possible = vec![];
        for i in 0..field_count {
            possible.push(i);
        }
        mapping.insert(c, possible);
    }

    for t in tickets {
        // urgghhhhh gave up with retain so have to do this...
        let mut updated: HashMap<&Criterion, Vec<usize>> = HashMap::new();
        for (c, fs) in &mapping {
            let mut valid = vec![];
            for f in fs {
                if !t.is_field_valid(c, *f) {
                    // println!("Ticket field {} invalid for {:?}\n{:?}", f, c, t);
                    // Whyyyyyyy
                    // fs.retain(|x| x != f);
                } else {
                    valid.push(*f);
                }
            }
            updated.insert(c, valid);
        }
        mapping = updated;
    }

    let mut found = 0;
    let mut check = 1;
    let mut exclude = vec![];
    let mut result = HashMap::new();
    while found < field_count {
        let mut fields = mapping.iter()
            .filter(|(_, v)| v.len() == check);
        
        while let Some((criterion, idxs)) = fields.next() {
            found += 1;
            let field_idx = idxs.iter()
                .filter(|f| !exclude.contains(f))
                .next();
            
            if let Some(idx) = field_idx {
                exclude.push(idx);
                result.insert(criterion.name.to_owned(), *idx);
                // println!("MATCH {}: {:#02}->{}", check, idx, criterion.name);
            } else {
                eprintln!("ERROR: ALL POSSIBILITIES EXCLUDED {}, {}->{:?}", check, criterion.name, idxs);
            }
            
        }

        check += 1;
    }

    result
}