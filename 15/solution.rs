fn main() {
    // println!("{}", calc(vec![1, 3, 2], 2020)); // 1
    // println!("{}", calc(vec![2, 1, 3], 2020)); // 10
    // println!("{}", calc(vec![1, 2, 3], 2020)); // 27
    
    // println!("{}", calc(vec![1, 3, 2], 30000000)); // 2578
    // println!("{}", calc(vec![2, 1, 3], 30000000)); // 3544142
    // println!("{}", calc(vec![1, 2, 3], 30000000)); // 261214
    
    let starting_numbers = vec![1, 0, 18, 10, 19, 6];

    // task 1
    let final_idx = 2020;
    println!("Starting numbers:\n{:?}", &starting_numbers);
    println!("Value at index {}:", final_idx);
    println!("{}", calc(&starting_numbers, final_idx));

    // task 2
    let final_idx = 30000000;
    println!("Starting numbers:\n{:?}", &starting_numbers);
    println!("Value at index {}:", final_idx);
    println!("{}", calc(&starting_numbers, final_idx));
}

fn calc(starting: &Vec<usize>, final_idx: usize) -> usize {
    let mut idx = 0;
    let mut last = 0;
    let mut track: Vec<i32> = vec![-1; final_idx];

    for val in starting {
        track[*val as usize] = idx as i32;
        
        last = *val;
        idx += 1;
    }
    
    track[last] = -1;
    
    while idx < final_idx {
        let current;
        let last_idx = track[last];
        if last_idx == -1 {
            current = 0;
        } else {
            current = idx - 1 - last_idx as usize;
        }
        track[last] = (idx - 1) as i32;
        last = current;

        idx += 1;
    }

    last
}