fn main() {
    // let args: Vec<String> = env::args().collect();
    // let input = "389125467"; // test
    let input = "589174263";
    play_game(input, 100, false);
    play_game(input, 10_000_000, true);

}

fn play_game(input: &str, iters: usize, big_game: bool) {
    let mut cups: Vec<usize> = input.chars()
        .map(String::from)
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let high_cup = *cups.iter().max().unwrap();

    if big_game {
        for i in (high_cup + 1)..=1_000_000 {
            cups.push(i);
        }
    }

    let mut buff = vec![];
    let cup_count = cups.len();

    let mut cup_idx = 0;
    for its in 0..iters {
        if its % 1000 == 0 {
            println!("Completed {} iterations", its);
        }
        // println!("");
        let cup = cups.get(cup_idx).unwrap().clone();
        // println!("Current cup is {} at index {}\n({:?})", cup, cup_idx, cups);
        let mut removal_idx = (cup_idx + 1) % cup_count;
        for _ in 0..3 {
            if removal_idx >= cups.len() {
                removal_idx = 0;
            }
            if removal_idx < cup_idx {
                cup_idx -= 1;
            }
            buff.push(cups.remove(removal_idx));
        }

        let mut target_cup = cup;
        loop {
            target_cup = if target_cup == 0 {
                high_cup
            } else {
                target_cup - 1
            };

            if let Some(idx) = cups.iter().position(|&c| c == target_cup) {
                if idx < cup_idx {
                    cup_idx = (cup_idx + 3) % cup_count;
                }
                // println!("Pick up: {:?}", buff);
                // println!("Destination: {} at index {}", target_cup, idx + 1);
                cups.splice((idx + 1)..(idx + 1), buff.clone());
                // cups.push(buff);
                buff.clear();
                // println!("After: {:?}", cups);
                break;
            }
        }
        cup_idx = (cup_idx + 1) % cup_count;
    }

    let one_idx = cups.iter().position(|&c| c == 1).unwrap();
    print!("Cup ring after {} rounds with ", iters);
    if big_game {
        print!("post 1 sequence: ");
        for i in 1..cup_count {
            print!("{}", cups.get((one_idx + i) % cup_count).unwrap());
        }
        println!("");
    } else {
        let next_cups = (
            cups.get((one_idx + 1) % cup_count).unwrap(),
            cups.get((one_idx + 2) % cup_count).unwrap()
        );
        print!("first 2 cups after 1 {}, {} product {}: ", next_cups.0, next_cups.1, next_cups.0 * next_cups.1);

    }
}