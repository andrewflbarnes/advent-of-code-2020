use std::env;
use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut player_1 = false;
    let mut p1_deck_start = VecDeque::new();
    let mut p2_deck_start = VecDeque::new();
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
                p1_deck_start.push_back(l.parse::<u8>().unwrap());
            } else {
                p2_deck_start.push_back(l.parse::<u8>().unwrap());
            }
        });

    let mut p1_deck = p1_deck_start.clone();
    let mut p2_deck = p2_deck_start.clone();

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

    let p1_deck = p1_deck_start.clone();
    let p2_deck = p2_deck_start.clone();
    let (winner, win_deck) = recurse_game(p1_deck, p2_deck);
    let (score, _) = win_deck.iter().rev().fold((0u64, 1u64), |(mut acc, mut mult), score| {
        acc += mult * *score as u64;
        mult += 1;
        (acc, mult)
    });
    println!("Player {} won with score {} and deck {:?}", winner, score, win_deck);
}

fn recurse_game(mut p1_deck: VecDeque<u8>, mut p2_deck: VecDeque<u8>) -> (u8, VecDeque<u8>) {
    let mut checks = HashSet::new();

    //  TODO return on repeated round
    while p1_deck.len() > 0 && p2_deck.len() > 0 {
        let p1_check = p1_deck.clone();
        let p2_check = p2_deck.clone();
        if !checks.insert((p1_check, p2_check)) {
            return (1, p1_deck);
        }

        let p1_card = &p1_deck.pop_front().unwrap();
        let p2_card = &p2_deck.pop_front().unwrap();

        if (*p1_card as usize) > p1_deck.len() || (*p2_card as usize) > p2_deck.len() {
            if p2_card > p1_card {
                p2_deck.extend(&[*p2_card, *p1_card]);
            } else {
                p1_deck.extend(&[*p1_card, *p2_card]);
            }
        } else {
            let p1_deck_r = p1_deck.iter().copied().take(*p1_card as usize).collect();
            let p2_deck_r = p2_deck.iter().copied().take(*p2_card as usize).collect();
            // println!("Recurse game with:");
            // println!("Player 1 ({}): {:?} => {:?}", p1_card, p1_deck, p1_deck_r);
            // println!("Player 2 ({}): {:?} => {:?}", p2_card, p2_deck, p2_deck_r);
            match recurse_game(p1_deck_r, p2_deck_r) {
                (1, _) => {
                    p1_deck.extend(&[*p1_card, *p2_card]);
                },
                (2, _) => {
                    p2_deck.extend(&[*p2_card, *p1_card]);
                },
                _ => panic!("... on the streets of London..."),
            }
        }
    }

    if p1_deck.len() > 0 {
        (1, p1_deck)
    } else {
        (2, p2_deck)
    }
}