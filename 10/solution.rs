use std::env;
use std::fs;
use std::iter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let expected_longest_one_peat = 5;
    
    let mut data: Vec<i32> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| {
            let val = line.parse::<i32>().unwrap();
            return val;
        })
        .collect();

    data.sort();
    // println!("{:?}", data);

    let (ones, threes, longest_one_peat, combinations) = process_data(&data);

    println!("Product of 1 jolt adapter increments ({}) and 3 jolt adapter increments ({}): {}", ones, threes, ones * threes);
    if longest_one_peat != expected_longest_one_peat {
        println!("Max expected 1 repeats of up to {} in length but got {}, next value is lower bound", expected_longest_one_peat, longest_one_peat);
    }
    println!("Number of combinations: {} (longest 1 jolt increment repetition: {})", combinations, longest_one_peat);

}

pub fn process_data(data: &Vec<i32>) -> (i32, i32, i32, u64) {
    // Add the final device which is always 3 jolts higher
    let final_device = data.last().unwrap_or(&0) + 3;
    let final_iter = iter::once(&final_device);

    let mut one_peat = 1;
    let mut longest_one_peat = 0;
    let mut combinations = 1;
    let (_, ones, threes) = data.iter()
        .chain(final_iter)
        // (last value, count 1 jolt increments, count 3 jolt increments)
        .fold((&0i32, 0i32, 0i32), |mut acc, val| {
            match val - acc.0 {
                1 => {
                    acc.1 += 1;
                    one_peat += 1;
                },
                3 => {
                    acc.2 += 1;
                    if one_peat > longest_one_peat {
                        longest_one_peat = one_peat;
                    }
                    match one_peat {
                        1 => { /* No additional combinations possible */}
                        2 => { /* No additional combinations possible */}
                        // Worke the below out with pen and paper after doing a run to calculate longest_one_peat
                        3 => combinations *= 2,
                        4 => combinations *= 4,
                        5 => combinations *= 7,
                        _ => { /* Error at bottom */},
                    }
                    one_peat = 1;
                },
                _ => println!("Invalid joltage drop: {} {} -> {}", acc.0, val, val - acc.0),
            }

            acc.0 = val;

            return acc;
        });
    return (ones, threes, longest_one_peat, combinations);
}

#[cfg(test)]
mod tests {
    #[test]
    fn process_empty() {
        let data = vec![];

        let (ones, threes, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 0);
        assert_eq!(threes, 1);
        assert_eq!(longest, 1);
        assert_eq!(combinations, 1);
    }
    
    #[test]
    fn process_trivial() {
        let data = vec![1];

        let (ones, threes, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 1);
        assert_eq!(threes, 1);
        assert_eq!(longest, 2);
        assert_eq!(combinations, 1);
    }
    
    #[test]
    fn process_three_ones() {
        let data = vec![1, 2];

        let (ones, _, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 2);
        assert_eq!(longest, 3);
        assert_eq!(combinations, 2);
    }
    
    #[test]
    fn process_four_ones() {
        let data = vec![1, 2, 3];

        let (ones, _, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 3);
        assert_eq!(longest, 4);
        assert_eq!(combinations, 4);
    }
    
    #[test]
    fn process_five_ones() {
        let data = vec![1, 2, 3, 4];

        let (ones, _, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 4);
        assert_eq!(longest, 5);
        assert_eq!(combinations, 7);
    }
    
    #[test]
    fn process_multiple_one_runs() {
        let data = vec![
            1, 2, 3, 4,
            7, 8, 9, 10,
            13, 14, 15,
            18, 19];

        let (ones, threes, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 10);
        assert_eq!(threes, 4);
        assert_eq!(longest, 5);
        assert_eq!(combinations, 56);
    }
    
    #[test]
    fn process_threes() {
        let data = vec![3, 6, 9, 12];

        let (ones, threes, longest, combinations) =  super::process_data(&data);

        assert_eq!(ones, 0);
        assert_eq!(threes, 5);
        assert_eq!(longest, 1);
        assert_eq!(combinations, 1);
    }
}