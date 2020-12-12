use std::env;
use std::fs;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut data: Vec<Vec<char>> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let rows = data.len() as i32;
    let cols = data[0].len() as i32;

    let mut buffer = data.clone();

    let mut iterations = 0;

    while data != buffer || iterations < 1{
        iterations += 1;
        // println!("Start next iteration: {}", iterations);
        for i in 0..rows {
            for j in 0..cols {
                let ii = i as usize;
                let jj = j as usize;
                let seat = data[ii][jj];
                if seat == '.' {
                    continue;
                }


                let mut adjacent = 0;
                for i_offset in -1..=1 {
                    for j_offset in -1..=1 {
                        if j_offset == 0 && i_offset == 0 {
                            continue;
                        }
                        adjacent += check_seat(rows, cols, i + i_offset, j + j_offset, &data);
                    }
                }

                // print!("({}, {}) {} -> {} -> ", i, j, adjacent, seat);

                if seat == 'L' && adjacent == 0 {
                    buffer[ii][jj] = '#';
                } else if seat == '#' && adjacent >= 4 {
                    buffer[ii][jj] = 'L';
                } else {
                    buffer[ii][jj] = seat;
                }

                // println!("{}", buffer[i][j]);
            }
        }
        mem::swap(&mut data, & mut buffer);
    }

    let mut occupied = 0;
    for i in 0..(rows as usize) {
        for j in 0..(cols as usize) {
            if data[i][j] == '#' {
                occupied += 1;
            }
        }
    }

    println!("Stable state after {} iterations with {} seats occupied", iterations, occupied);
}

fn check_seat(rows: i32, cols: i32, row: i32, col: i32, data: &Vec<Vec<char>>) -> i32 {
    if row < 0 || row >= rows || col < 0 || col >= cols {
        0
    } else if data[row as usize][col as usize] == '#' {
        1
    } else {
        0
    }
}