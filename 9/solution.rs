use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::{
    BufReader,
    BufRead
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let preamble = &args[2].parse::<i32>().unwrap();
    let preamble = 25;

    println!("Preamble: {}", preamble);

    let file = fs::File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut idx = 0;
    let buffer_size = preamble - 1;
    let mut val_buffer: VecDeque<i64> = VecDeque::with_capacity(buffer_size);
    let mut combo_buffer: VecDeque<Vec<i64>> = VecDeque::new();

    loop {
        line.clear();

        match buf.read_line(&mut line) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    // EOF
                    break;
                }

                // remove newline
                let this_val = line.trim().parse::<i64>().unwrap();

                if idx > preamble {
                    let mut found = false;
                    'outer: for vals in &combo_buffer {
                        for val in vals {
                            if *val == this_val {
                                found = true;
                                break 'outer;
                            }
                        }
                    }

                    if !found {
                        println!("{}: Value {} was not composable from previous {} entries:", idx + 1, this_val, preamble);
                        // println!("{:?}", val_buffer);
                        // println!("{:?}", combo_buffer);
                    }
                }

                let mut combos = vec![];
                for i in (0..val_buffer.len()).rev() {
                    let combo = this_val + val_buffer.get(i).unwrap();
                    combos.push(combo);
                }

                if idx > preamble {
                    for vals in &mut combo_buffer {
                        vals.pop();
                    }
                    val_buffer.pop_front();
                    combo_buffer.pop_front();
                }

                val_buffer.push_back(this_val);
                combo_buffer.push_back(combos);
            }
            Err(err) => {
                return Err(err);
            }
        }

        idx += 1;
    }

    Ok(())
}