use std::env;
use std::fs;

const MODULO: u64 = 20201227;
const HANDSHAKE_SN: u64 = 7;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let pubkeys = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    let sn = HANDSHAKE_SN;
    for (i, &pubkey) in pubkeys.iter().enumerate() {
        let iter = get_loop_size(sn, pubkey, MODULO);

        let enckey = transform(
            if i == 0 {pubkeys[1]} else {pubkeys[0]},
            iter,
            MODULO,
        );

        println!("Public key {} (index={}) reached after {} iterations - encryption key {}", pubkey, i, iter, enckey);
    }
}

fn get_loop_size(sn: u64, pubkey: u64, modulo: u64) -> u64 {
    let mut gen_pubkey = 1;
    let mut iter = 0;
    loop {
        iter += 1;
        gen_pubkey = (gen_pubkey * sn) % modulo;
        if gen_pubkey == pubkey {
            break;
        }
    }
    iter
}

fn transform(sn: u64, loops: u64, modulo: u64) -> u64 {
    let mut res = 1;
    for _ in 0..loops { res = (res * sn) % modulo };
    res
}