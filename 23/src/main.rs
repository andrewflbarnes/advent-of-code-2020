fn main() {
    // let args: Vec<String> = env::args().collect();
    // let input = "389125467"; // test
    let input = "589174263";
    let mut cups: Vec<usize> = input.chars()
        .map(String::from)
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    let high_cup = *cups.iter().max().unwrap();
    let mut buff = vec![];
    let cup_count = cups.len();
    let iters = 100;

    let mut cup_idx = 0;
    for _ in 0..iters {
        println!("");
        let cup = cups.get(cup_idx).unwrap().clone();
        println!("Current cup is {} at index {}\n({:?})", cup, cup_idx, cups);
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
                println!("Pick up: {:?}", buff);
                println!("Destination: {} at index {}", target_cup, idx + 1);
                cups.splice((idx + 1)..(idx + 1), buff.clone());
                buff.clear();
                println!("After: {:?}", cups);
                break;
            }
        }
        cup_idx = (cup_idx + 1) % cup_count;
    }

    let one_idx = cups.iter().position(|&c| c == 1).unwrap();
    print!("Cup ring after {} rounds with post 1 sequence: ", iters);
    for i in 1..cup_count {
        print!("{}", cups.get((one_idx + i) % cup_count).unwrap());
    }
    println!("");

}
