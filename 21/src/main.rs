use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input: Vec<String> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect();

    let mut ingredients = vec![];
    let mut allergens: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.iter() {
        let line = l.replace(")", "");
        let mut atoms = line.split(" (contains ");
        // println!("{}", atoms.next().unwrap());
        // println!("{}", atoms.next().unwrap());
        let mut these_ingredients = vec![];
        atoms.next()
            .unwrap()
            .split(" ")
            .map(String::from)
            .for_each(|i| {
                if !ingredients.contains(&i) {
                    ingredients.push(i.clone());
                }
                these_ingredients.push(i);
            });

        atoms.next()
            .unwrap()
            .split(", ")
            .map(String::from)
            .for_each(|a| {
                match allergens.get_mut(&a) {
                    Some(ingrs) => {
                        let updated = ingrs.iter()
                            .filter(|i| these_ingredients.contains(i))
                            .map(String::from)
                            .collect();
                        allergens.insert(a, updated);
                    },
                    None => {
                        allergens.insert(a.clone(), these_ingredients.clone());
                    },
                }
            });
    }

    // println!("{:?}", ingredients);
    // println!("{:?}", allergens);

    let mut allergen_ingrs = HashMap::new();
    let mut solved = 0;
    let allergen_count = allergens.len();

    while solved < allergen_count {
        allergens.iter_mut()
            .for_each(|(k, v)| {
                v.retain(|i| !allergen_ingrs.contains_key(i));
                if v.len() == 1 {
                    // println!("Solved {} with {:?}", k, v);
                    allergen_ingrs.insert(v.pop().unwrap(), k.clone());
                    solved += 1;
                }
            });
    }

    let mut non_allergens = ingredients.clone();
    non_allergens.retain(|i| !allergen_ingrs.contains_key(i));
    let count = input.iter()
        .fold(0, |mut acc, line| {
            acc += line.split(" ")
                .map(String::from)
                .filter(|v| non_allergens.contains(v))
                .count();
            acc
        });

    println!("Occurrences of non-allergens: {}", count);

    print!("Dangerous ingredient list: ");
    let mut allergens_ordered = allergen_ingrs.iter()
        .map(|(_, v)| String::from(v))
        .collect::<Vec<String>>();
    allergens_ordered.sort();

    let dangerous_ingredients = allergens_ordered.iter()
        .map(|a| allergen_ingrs.iter().filter(|(_, v)| v == &a).map(|(k, _)| k).next().unwrap())
        .map(String::from)
        .collect::<Vec<String>>();

    println!("{}", dangerous_ingredients.join(","));
}