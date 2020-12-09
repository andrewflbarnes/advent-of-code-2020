use std::env;
use std::fs;
use std::io::{
    BufReader,
    BufRead
};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let target = args[2].parse::<i64>().unwrap();

    let file = fs::File::open(filename)?;
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let mut val_buffer: Vec<i64> = vec![];
    let mut total = 0;

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
                val_buffer.push(this_val);

                total += this_val;

                let mut reduction_count = 0;
                while total > target {
                    total -= val_buffer[reduction_count];
                    reduction_count += 1;
                }

                if reduction_count > 0 {
                    val_buffer = val_buffer[reduction_count..].to_vec();
                }

                if total == target {
                    let max = val_buffer.iter().max().unwrap();
                    let min = val_buffer.iter().min().unwrap();
                    println!("Contiguous set summing to {} found with min={}, max={} summing to {}", target, min, max, min + max);
                    break;
                }


            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}