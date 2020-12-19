use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    id: String,
    condition: String,
}

impl Rule {
    fn compile(&self, rules: &HashMap<String, Rule>, regex: &mut HashMap<String, String>) {
        let mut atoms = self.condition.split(" ");
        let mut rule_regex = "".to_string();
        let mut piped = false;
        let enable_t2 = true;
        let t2_r11_size = 8;
        let mut t2_r8 = false;
        if enable_t2 && self.id == "8" {
            t2_r8 = true;
        }
        let mut t2_r11 = false;
        let mut t2_r11_vals: (String, String) = ("".to_string(), "".to_string());
        if enable_t2 && self.id == "11" {
            t2_r11 = true;
        }
        while let Some(atom) = atoms.next() {
            match atom {
                sub_rule_id if sub_rule_id.chars().all(char::is_numeric) => {
                    match regex.get(sub_rule_id) {
                        Some(r) => {
                            rule_regex.push_str(&r);
                            if t2_r11 {
                                if t2_r11_vals.0 == "" {
                                    t2_r11_vals.0 = r.clone();
                                } else {
                                    t2_r11_vals.1 = r.clone();
                                }
                            }
                        },
                        None => {
                            rules.get(sub_rule_id).unwrap().compile(rules, regex);
                            let r = regex.get(sub_rule_id).unwrap();
                            rule_regex.push_str(r);
                            if t2_r11 {
                                if t2_r11_vals.0 == "" {
                                    t2_r11_vals.0 = r.clone();
                                } else {
                                    t2_r11_vals.1 = r.clone();
                                }
                            }
                        },
                    };
                },
                "|" => {
                    rule_regex.push_str("|");
                    piped = true;
                },
                _ => rule_regex.push_str(&atom[1..(atom.len() - 1)]),
            }
        }

        if piped {
            let mut contained = "(".to_string();
            contained.push_str(&rule_regex);
            contained.push_str(")");
            rule_regex = contained;
        }

        if t2_r8 {
            rule_regex = format!("({})+", rule_regex);
        }
        if t2_r11 {
            rule_regex = String::from("(");
            for i in 1..=t2_r11_size {
                rule_regex.push_str(&"(");
                rule_regex.push_str(&t2_r11_vals.0);
                rule_regex.push_str(&format!("){{{}}}(", i));
                rule_regex.push_str(&t2_r11_vals.1);
                rule_regex.push_str(&format!("){{{}}}", i));
                if i != t2_r11_size {
                    rule_regex.push_str(&"|");
                }
            }
            rule_regex.push_str(&")");
            // rule_regex = format!("({})+({})+", t2_r11_vals.0, t2_r11_vals.1);
        }
        // self.regex = Some(rule_regex.to_owned());
        regex.insert(self.id.to_owned(), rule_regex);
    }
}

impl FromStr for Rule {
    type Err = String;
    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        let mut atoms = s.split(": ")
            .map(String::from);

        Ok(Rule{
            id: atoms.next().unwrap(),
            condition: atoms.next().unwrap(),
        })
    }
}

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

    let mut regex = HashMap::new();
    let rule = &rules.get(match_rule).unwrap();
    rule.compile(&rules, &mut regex);

    let rule_regex = &format!("^{}$", regex.get(match_rule).unwrap());
    // println!("{:?}", rule_regex);

    let matcher = Regex::new(rule_regex).unwrap();
    let matched = strings.iter()
        .filter(|s| {
            let res = matcher.is_match(s);

            if res {
                // println!("Matched: {}", s);
            }
            res
        })
        .count();
    println!("Strings matching rule {}: {}", match_rule, matched);
}

// 4 a
// 5 b
// 3 ab|ba
// 2 aa|bb
// 
// 