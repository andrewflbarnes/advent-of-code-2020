use std::env;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

mod rules;
use rules::Rule;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let match_rule = &args[2];

    let input: Vec<String> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect();
    
    let mut is_rules = true;
    let (rules, strings) = input.iter()
        .fold((HashMap::new(), vec![]), |(mut rules, mut strings), line| {
            if line.len() == 0 {
                is_rules = false;
            } else if is_rules {
                let rule = line.parse::<Rule>().unwrap();
                rules.insert(rule.id.to_owned(), rule);
            } else {
                strings.push(line.to_owned());
            }
            (rules, strings)
        });

    // task 1
    process(&strings, &rules, match_rule, false);

    // task 2
    process(&strings, &rules, match_rule, true);
}

fn process(strings: &Vec<String>, rules: &HashMap<String, Rule>, match_rule: &str, task_2: bool) {
    let regex = rules::compile(rules, task_2);
    let rule_regex = &format!("^{}$", regex.get(match_rule).unwrap());
    let matcher = Regex::new(rule_regex).unwrap();

    let matched = strings.iter()
        .filter(|s| matcher.is_match(s))
        .count();
    println!("Strings matching rule {}: {}", match_rule, matched);
}