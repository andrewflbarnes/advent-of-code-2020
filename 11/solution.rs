use std::env;
use std::fs;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut rows = 0;
    let mut cols = 0;
    
    let mut data: Vec<Vec<char>> = vec![];
    
    fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .for_each(|line| {
            data.push(line.chars().collect());
            rows += 1;
            cols = line.len();
        });

    println!("rows={} cols={}", rows, cols);
    // println!("data={:?}", data);

    let mut buffer = data.clone();

    let mut iterations = 0;

    while data != buffer || iterations < 1{
        iterations += 1;
        // println!("Start next iteration: {}", iterations);
        for i in 0..rows {
            for j in 0..cols {
                let seat = data[i][j];
                if seat == '.' {
                    continue;
                }


                let mut adjacent = 0;
                let is = i.checked_sub(1);
                let ia = Some(i + 1);
                let ii = Some(i);
                let js = j.checked_sub(1);
                let ja = Some(j + 1);
                let jj = Some(j);
                adjacent += check_seat(rows, cols, is, js, &data);
                adjacent += check_seat(rows, cols, is, jj, &data);
                adjacent += check_seat(rows, cols, is, ja, &data);
                adjacent += check_seat(rows, cols, ii, js, &data);
                adjacent += check_seat(rows, cols, ii, ja, &data);
                adjacent += check_seat(rows, cols, ia, js, &data);
                adjacent += check_seat(rows, cols, ia, jj, &data);
                adjacent += check_seat(rows, cols, ia, ja, &data);

                // print!("({}, {}) {} -> {} -> ", i, j, adjacent, seat);

                if seat == 'L' && adjacent == 0 {
                    buffer[i][j] = '#';
                } else if seat == '#' && adjacent >= 4 {
                    buffer[i][j] = 'L';
                } else {
                    buffer[i][j] = seat;
                }

                // println!("{}", buffer[i][j]);
            }
        }
        mem::swap(&mut data, & mut buffer);
    }

    let mut occupied = 0;
    for i in 0..rows {
        for j in 0..cols {
            if data[i][j] == '#' {
                occupied += 1;
            }
        }
    }

    println!("Stable state after {} iterations with {} seats occupied", iterations, occupied);
}

fn check_seat(rows: usize, cols: usize, row: Option<usize>, col: Option<usize>, data: &Vec<Vec<char>>) -> i32 {
    if row == None || col == None {
        return 0;
    }

    let rr = row.unwrap();
    let cc = col.unwrap();

    if rr >= rows || cc >= cols {
        0
    } else if data[rr][cc] == '#' {
        1
    } else {
        0
    }
}