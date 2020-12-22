use std::env;
use std::fs;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut player_1 = false;
    let mut p1_deck = VecDeque::new();
    let mut p2_deck = VecDeque::new();
    fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .for_each(|l| {
            if l.len() == 0 {
                return;
            }

            if l.starts_with("Player") {
                player_1 = !player_1;
            } else if player_1 {
                p1_deck.push_back(l.parse::<u8>().unwrap());
            } else {
                p2_deck.push_back(l.parse::<u8>().unwrap());
            }
        });

    while p1_deck.len() > 0 && p2_deck.len() > 0 {
        let p1_card = &p1_deck.pop_front().unwrap();
        let p2_card = &p2_deck.pop_front().unwrap();

        if p2_card > p1_card {
            p2_deck.push_back(*p2_card);
            p2_deck.push_back(*p1_card);
        } else {
            p1_deck.push_back(*p1_card);
            p1_deck.push_back(*p2_card);

        }
    }

    let (winner, (score, _)) = match p1_deck.len() {
        0 => ("2", p2_deck.iter().rev().fold((0u64, 1u64), |(mut acc, mut mult), score| {
            acc += mult * *score as u64;
            mult += 1;
            (acc, mult)
        })),
        _ => ("1", p1_deck.iter().rev().fold((0u64, 1u64), |(mut acc, mut mult), score| {
            acc += mult * *score as u64;
            mult += 1;
            (acc, mult)
        })),
    };

    println!("Player {} won with score {}", winner, score);
}