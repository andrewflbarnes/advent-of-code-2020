use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    pub id: String,
    condition: String,
}

impl Rule {
    fn compile(&self, rules: &HashMap<String, Rule>, regex: &mut HashMap<String, String>, enable_t2: bool) {
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

        let mut atoms = self.condition.split(" ");
        let mut rule_regex = "".to_string();
        let mut piped = false;
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
                            rules.get(sub_rule_id).unwrap().compile(rules, regex, enable_t2);
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
        }

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

pub fn compile(rules: &HashMap<String, Rule>, enable_t2: bool) -> HashMap<String, String> {
    let mut regex = HashMap::new();
    for (id, rule) in rules.iter() {
        if !regex.contains_key(id) {
            rule.compile(&rules, &mut regex, enable_t2);
        }
    }

    regex
}