use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let seats: Vec<i32> = fs::read_to_string(filename)
        .expect("Unable to read input file")
        .lines()
        .map(|line| seat_id(&line))
        .collect();

    println!("Max ID: {}", seats.iter().max().unwrap());

    let l_limit: i32 = 10;
    let u_limit: i32 = 127 * 8;
    let mut prev: bool;
    let mut maybe: bool = seats.contains(&(l_limit - 2));
    let mut post: bool = seats.contains(&(l_limit - 1));
    let mut possible_seats = vec![];

    for i in l_limit..u_limit {
        prev = maybe;
        maybe = post;
        post = seats.contains(&i);
        if prev && post & !maybe {
            possible_seats.push(i - 1);
        }
    }

    println!("Possible seats: {:?}", possible_seats);
}

fn seat_id(s: &str) -> i32 {
    (parse_bin(&s[..7]) * 8) + parse_bin(&s[7..])
}

fn parse_bin(s: &str) -> i32 {
    let len = s.len();
    let mut val = 0;

    for (i, c) in s.chars().enumerate() {
        if c == 'R' || c == 'B' {
            val += 1 << (len - i - 1);
        }
    }

    val
}