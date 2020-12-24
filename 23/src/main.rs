fn main() {
    // let args: Vec<String> = env::args().collect();
    // let input = "389125467"; // test
    let input = "589174263";
    // play_game(input, 10, false);
    play_game(input, 100, false);
    play_game(input, 10_000_000, true);

}

fn play_game(input: &str, iters: usize, big_game: bool) {
    let mut cups: Vec<usize>;
    if big_game {
        cups = vec![0; 1_000_001];
        // cups = vec![0; 1_001];
    } else {
        cups = vec![0; input.len() + 1];
    }

    let first_cup = input[0..1].parse::<usize>().unwrap();
    let init = input.chars()
        .map(String::from)
        .map(|c| c.parse::<usize>().unwrap());
    let mut last = init
        .fold(0, |last, val| {
            if last != 0 {
                cups[last] = val;
            }
            val
        });
    
    for i in (input.len() + 1)..cups.len() {
        cups[last] = i;
        last = i;
    }

    let high_cup = *cups.iter().max().unwrap();
    cups[last] = first_cup;

    // let mut cup = first_cup;
    // for _ in 0..(cups.len() - 1) {
    //     print!("Cup {} ", cup);
    //     cup = cups[cup];
    //     println!("points to {}", cup);
    // }
    // println!("From {} to {}", first_cup, cup);

    let mut buff = vec![];

    let mut cup_idx = first_cup;
    for its in 0..iters {
        // clear check buffer
        buff.clear();

        // the number of the current cup
        let current_cup = cup_idx;
        // the number of the next cup (which we start the cut from)
        let cut_start = cups[cup_idx];
        buff.push(cut_start);
        // the second cup we cut - no standalone variable name as we don't need it later outside the buff
        cup_idx = cups[cut_start];
        buff.push(cup_idx);
        // the third and final cup we cut
        let cut_end = cups[cup_idx];
        buff.push(cut_end);
        // the cup to use for the next iteration
        cup_idx = cups[cut_end];
        // println!("source: {}", current_cup);
        // print!("cups:");
        // let mut temp = first_cup;
        // for _ in 1..cups.len() {
        //     print!(" {}", temp);
        //     temp = cups[temp];
        // }
        // println!("");
        // println!("pick up: {:?}", buff);

        let mut target_cup = current_cup;
        loop {
            target_cup = if target_cup == 1 {
                high_cup
            } else {
                target_cup - 1
            };

            if !buff.contains(&target_cup) {
                // println!("destination: {}\n", target_cup);
                cups[current_cup] = cups[cut_end];
                cups[cut_end] = cups[target_cup];
                cups[target_cup] = cut_start;
                break;
            }
        }
    }

    
    let after_one = cups[1];
    print!("Cup ring after {} rounds with ", iters);
    if !big_game {
        print!("post 1 sequence: ");
        let mut temp = after_one;
        for _ in 2..cups.len() {
            print!("{}", temp);
            temp = cups[temp];
        }
        println!("");
    } else {
        print!("first 2 cups {}, {} product {}: ", after_one, cups[after_one], after_one * cups[after_one]);

    }
}