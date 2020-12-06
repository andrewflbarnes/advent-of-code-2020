use std::env;
use std::fs;
use std::collections::HashSet;
use std::io::{
    BufReader,
    BufRead,
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let strict: bool;
    // if args.len() > 2 {
    //     strict = args[2] == "strict";
    // } else {
    //     strict = false;
    // }
    // match strict {
    //     true  => println!("Strict processing enabled"),
    //     false => println!("Strict processing disabled"),
    // }

    let file = fs::File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut answers_any: HashSet<char> = HashSet::new();
    let mut answers_all: HashSet<char> = HashSet::new();
    let mut group_answer_any = 0;
    let mut group_answer_all = 0;
    let mut new_group = true;

    loop {
        match buf.read_line(&mut line) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    // EOF
                    group_answer_any += answers_any.len();
                    group_answer_all += answers_all.len();
                    break;
                }

                // remove newline
                line = line.trim().into();
                if !line.is_empty() {
                    let these_answers = line.chars().collect();

                    answers_any.extend(&these_answers);

                    if new_group {
                        answers_all.extend(&these_answers);
                        new_group = false;
                    } else {
                        answers_all = answers_all
                            .intersection(&these_answers)
                            .map(|c| *c)
                            .collect();
                    }
                } else {
                    group_answer_any += answers_any.len();
                    group_answer_all += answers_all.len();
                    answers_any.clear();
                    answers_all.clear();
                    new_group = true;
                }

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    println!("Group answer sum (any yes): {}", group_answer_any);
    println!("Group answer sum (all yes): {}", group_answer_all);

    Ok(())
}