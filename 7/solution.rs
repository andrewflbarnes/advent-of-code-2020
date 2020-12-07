use std::env;
use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Bag {
    info: String,
    color: String,
    contents: Vec<(String, i32)>,
    owners: Vec<String>,
}

impl FromStr for Bag {
    type Err = String;
    fn from_str(s: &str) -> Result<Bag, Self::Err> {
        let info = s.into();
        let mut contents: Vec<(String, i32)> = vec![];
        
        let mut clauses = s.split(" bags contain ");
        let color = clauses.next().unwrap().into();

        let possible_contents = clauses.next().unwrap();
        if possible_contents != "no other bags." {
            for bag in possible_contents.split(", ") {
                let last_space = bag.rfind(" ").unwrap();
                let mut bag_atoms = bag[..last_space].splitn(2, " ");
                let bag_count = bag_atoms.next().unwrap().parse::<i32>().unwrap();
                let bag_color = bag_atoms.next().unwrap().into();
                contents.push((
                    bag_color,
                    bag_count,
                ));
            }
        }
        
        Ok(Bag{
            info,
            color,
            contents,
            owners: vec![],
        })
    }
}

impl Bag {
    fn add_owner(&mut self, owner: String) {
        if !self.owners.contains(&owner) {
            self.owners.push(owner);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut bags: HashMap<String, Bag> = HashMap::new();
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    
    fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .for_each(|line| {
            let bag = line.parse::<Bag>().unwrap();
            bags.insert(bag.color.clone(), bag);
        });

    let clone = bags.clone();
    let bag_vals: Vec<&Bag> = clone.values().collect();
    for bag in &bag_vals {
        links.entry(&bag.color).or_insert(vec![]);
        for child in &bag.contents {
            bags.get_mut::<str>(&child.0).unwrap().add_owner(bag.color.clone());
            links.entry(&child.0).or_insert(vec![]);
            links.get_mut::<str>(&child.0).unwrap().push(&bag.color);
        }
    }

    let color_check = "shiny gold";

    let containing_bags = can_contain(color_check, &links);
    println!("{} bags containing {} bags", containing_bags.len(), color_check);
    // println!("{:?}", containing_bags);

    let count = nested_bags(color_check, &bags, 1);
    println!("{} bags contained by a single {} bag", count, color_check);
}

fn can_contain<'a>(color: &str, bags: &'a HashMap<&str, Vec<&str>>) -> Vec<&'a str> {
    let mut matched: Vec<&str> = vec![];
    let mut checked: Vec<&str> = vec![];
    let mut check: Vec<&str> = vec![];

    check.push(color);

    while let Some(check_color) = check.pop() {
        if !checked.contains(&check_color) {
            checked.push(check_color.clone());

            let check_bag = bags.get(check_color).unwrap();

            for owner_color in check_bag {
                if !checked.contains(owner_color) {
                    check.push(owner_color);
                }
                if !matched.contains(owner_color) {
                    matched.push(owner_color);
                }
            }
        }
    }

    matched
}

fn nested_bags(color: &str, bags: &HashMap<String, Bag>, number_bags: i32) -> i32 {
    let mut count = 0;
    do_nested_bags(color, bags, number_bags, &mut count, &mut HashMap::new());
    count
}

fn do_nested_bags(color: &str, bags: &HashMap<String, Bag>, number_bags: i32, count: &mut i32, shortcut: &mut HashMap<String, i32>) {
    let bag = bags.get(color).unwrap();

    if bag.contents.len() == 0 {
        return;
    }

    for child_bag in &bag.contents {
        // println!("{} {} bag contains {} {} bags")
        *count += number_bags * child_bag.1;
        do_nested_bags(&child_bag.0, bags, child_bag.1 * number_bags, count, shortcut);
    }
}