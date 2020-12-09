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
    let mut buffer_idx = 0;
    let buffer_size = preamble - 1;
    let mut val_buffer: Vec<i64> = vec![0; buffer_size];
    let mut combo_buffer: Vec<Vec<i64>> = vec![vec![]; buffer_size];
    let mut entries = 0;

    loop {
        // println!("entries={}, idx={} buffer_idx={} buffer={:?}", entries, idx, buffer_idx, val_buffer);
        line.clear();

        match buf.read_line(&mut line) {
            Ok(bytes_read) => {

                if bytes_read == 0 {
                    // EOF
                    break;
                }

                // remove newline
                let this_val = line.trim().parse::<i64>().unwrap();
                val_buffer[buffer_idx] = this_val;

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
                        println!("{:?}", combo_buffer);
                    }
                }

                let mut combos = vec![];
                for i in 0..entries {
                    let entry_idx;
                    if i > buffer_idx {
                        entry_idx = buffer_idx + entries - i;
                    } else {
                        entry_idx = buffer_idx - i;
                    }
                    let combo = this_val + val_buffer[entry_idx];
                    combos.push(combo);
                }

                combo_buffer[buffer_idx] = combos;
            }
            Err(err) => {
                return Err(err);
            }
        }
        
        idx += 1;
        if buffer_idx == buffer_size - 1 {
            buffer_idx = 0;
        } else {
            buffer_idx += 1;
        }
        if entries < buffer_size {
            entries += 1;
        }
    }

    Ok(())
}