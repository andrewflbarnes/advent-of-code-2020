use std::env;
use std::fs;
use std::io::{
    BufReader,
    BufRead,
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let slopes = [
        ( 1, 1 ),
        ( 1, 3 ),
        ( 1, 5 ),
        ( 1, 7 ),
        ( 2, 1 ),
    ];

    let product = slopes.iter().map(|s| {
        let file = fs::File::open(filename).unwrap();
        let mut buf = BufReader::new(file);
        let mut line = String::new();
        let mut line_no = 0;
        let mut hit = 0;
        let mut x = 0;
        let mut width = 0;

        loop {
            match buf.read_line(&mut line) {
                Ok(bytes_read) => {
                    if width == 0 {
                        // exclude newline
                        width = line.len() - 1;
                    }
                    // println!("{} -> {}", x, line);
                    if line_no % s.0 == 0 {
                        if bytes_read == 0 {
                            println!("{} trees hit for xInc {} yInc {} on {} lines", hit, s.1, s.0, line_no);
                            return hit;
                        } else {
                            let chars: Vec<char> = line.chars().collect();
                            if chars[x] == '#' {
                                // println!("hit");
                                hit += 1;
                            }
                        }

                        x += s.1;
                        if x >= width {
                            x -= width;
                        }
                    }

                    line.clear();
                    line_no += 1;
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    return 0;
                }
            }
        }
    })
    .fold(1i32, |sum, value| sum * value);

    println!("{}", product);

    Ok(())
}