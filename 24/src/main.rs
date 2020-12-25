use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let instrs = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
    instrs.iter().for_each(|i| {
        let mut instr = String::from("");
        let mut tile = (0, 0);
        i.chars().for_each(|c| {
            instr.push(c);
            match &instr[..] {
                "e" => tile.1 += 1,
                "se" => tile.0 += 1,
                "sw" => {
                    tile.0 += 1;
                    tile.1 -= 1;
                },
                "w" => tile.1 -= 1,
                "nw" => tile.0 -= 1,
                "ne" => {
                    tile.0 -= 1;
                    tile.1 += 1;
                },
                "s" | "n" => return,
                _ => panic!("{}", instr),
            }
            instr.clear();
        });

        if black_tiles.contains(&tile) {
            black_tiles.retain(|&t| t != tile);
        } else {
            black_tiles.insert(tile);
        }
    });

    let iters = 100;
    let mut buff:HashSet<(i32, i32)> = HashSet::new();
    println!("Day 0: {}", black_tiles.len());
    for _ in 1..=iters {
        buff.clear();
        for tile in black_tiles.iter() {
            let adj = count_adj(&black_tiles, tile);
            if adj > 0 && adj <= 2 {
                buff.insert(*tile);
            }

            push_if_black((tile.0, tile.1 + 1), &black_tiles, &mut buff);
            push_if_black((tile.0, tile.1 - 1), &black_tiles, &mut buff);
            push_if_black((tile.0 + 1, tile.1), &black_tiles, &mut buff);
            push_if_black((tile.0 - 1, tile.1), &black_tiles, &mut buff);
            push_if_black((tile.0 + 1, tile.1 - 1), &black_tiles, &mut buff);
            push_if_black((tile.0 - 1, tile.1 + 1), &black_tiles, &mut buff);
        }
        black_tiles = buff.clone();
    }
    
    println!("Day {}: {}", iters, black_tiles.len());
}

fn push_if_black( tile: (i32, i32), black_tiles: &HashSet<(i32, i32)>,buff: &mut HashSet<(i32, i32)>) {
    if black_tiles.contains(&tile) || buff.contains(&tile) {
        return;
    }
    let adj = count_adj(&black_tiles, &tile);
    if black_tiles.contains(&tile) {
        if adj > 0 && adj <= 2 {
            buff.insert(tile);
        }
    } else {
        if adj == 2 {
            buff.insert(tile);
        }
    }
}

fn count_adj(black_tiles: &HashSet<(i32, i32)>, tile: &(i32, i32)) -> i32 {
    let mut adj = 0;
    adj += if black_tiles.contains(&(tile.0, tile.1 + 1)) {1} else {0};
    adj += if black_tiles.contains(&(tile.0, tile.1 - 1)) {1} else {0};
    adj += if black_tiles.contains(&(tile.0 + 1, tile.1)) {1} else {0};
    adj += if black_tiles.contains(&(tile.0 - 1, tile.1)) {1} else {0};
    adj += if black_tiles.contains(&(tile.0 + 1, tile.1 - 1)) {1} else {0};
    adj += if black_tiles.contains(&(tile.0 - 1, tile.1 + 1)) {1} else {0};
    adj
}