use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let instrs = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    
    let mut black_tiles = vec![];
    instrs.iter().for_each(|i| {
        let mut instr = String::from("");
        let mut tile = (0, 0);
        i.chars().for_each(|c| {
            instr.push(c);
            // print!("{:?} => ", tile);
            match &instr[..] {
                "e" => tile.0 += 1,
                "se" => tile.1 += 1,
                "sw" => {
                    tile.0 -= 1;
                    tile.1 += 1;
                },
                "w" => tile.0 -= 1,
                "nw" => tile.1 -= 1,
                "ne" => {
                    tile.0 += 1;
                    tile.1 -= 1;
                },
                "s" | "n" => return,
                _ => panic!("{}", instr),
            }
            // println!("{:?} ({})", tile, instr);
            instr.clear();
        });

        // print!("{} {:?} => ", black_tiles.len(), tile);
        if black_tiles.contains(&tile) {
            black_tiles.retain(|&t| t != tile);
        } else {
            black_tiles.push(tile);
        }
        // println!("{}", black_tiles.len());
    });

    println!("Black tile count: {}", black_tiles.len());
    // black_tiles.iter().for_each(|t| {println!("{:?}", t)});
}