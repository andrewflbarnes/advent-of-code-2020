use std::env;
use std::fs;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
    id: u16,
    image: Vec<String>,
    borders: Vec<String>,
}

impl Tile {
    fn new(id: &str) -> Tile {
        let start = id.find(" ").unwrap() + 1;
        let end = id.find(":").unwrap();
        let id_num = id[start..end].parse::<u16>().unwrap();
        Tile {
            id: id_num,
            image: vec![],
            borders: vec![],
        }
    }

    fn add_image_line(&mut self, s: &str) {
        self.image.push(String::from(s));
    }

    fn calculate_borders(&mut self) {
        // first 4 are the "standard" oriented borders (clockwise direction) clockwise from top
        let mut border_r = String::new();
        let mut border_l = String::new();
        let mut border_r_rev = String::new();
        let mut border_l_rev = String::new();

        let image_len = self.image.len();
        let last = self.image[0].len() - 1;
        for i in 0..image_len {
            let line_f = &self.image[i];
            let line_b = &self.image[image_len -1 - i];
            border_r.push_str(&line_f.as_str()[last..]);
            border_l.push_str(&line_b.as_str()[0..1]);
            border_r_rev.push_str(&line_b.as_str()[last..]);
            border_l_rev.push_str(&line_f.as_str()[0..1]);
        }

        // clockwise ordering with "standard" (clockwise) orientation
        self.borders.push(self.image[0].clone());
        self.borders.push(border_r);
        self.borders.push(self.image[image_len - 1].chars().rev().collect());
        self.borders.push(border_l);
        
        // clockwise ordering with "reverse" (flipped/anti-clockwise) orientation
        self.borders.push(self.image[0].chars().rev().collect());
        self.borders.push(border_r_rev);
        self.borders.push(self.image[image_len - 1].clone());
        self.borders.push(border_l_rev);
    }

    fn adjacent_to(&self, other: &Self) -> bool {
        for i in 0..4 {
            for other_border in other.borders.iter() {
                if self.id == 2729 || other.id == 2729 {
                    println!("Check:");
                    println!("{} -> {}", self.id, self.borders[i]);
                    println!("{} -> {}", other.id, other_border);
                }
                if self.borders[i] == other_border.to_owned() {
                    return true;
                }
            }
        }
        false
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.id);
        for line in self.image.iter() {
            writeln!(f, "{}", line);
        }
        writeln!(f, "Borders:");
        for line in self.borders.iter() {
            writeln!(f, "{}", line);
        }
        Ok(())
    }
}

fn find_corners(tiles: &Vec<Tile>) -> Vec<u16> {
    let mut matches: HashMap<u16, u8> = HashMap::new();

    for i in 0..(tiles.len()) {
        let tile = &tiles[i];
        for j in (i + 1)..tiles.len() {
            let check_tile = &tiles[j];
            if tile.adjacent_to(&check_tile) {
                match matches.get_mut(&tile.id) {
                    Some(v) => *v += 1,
                    None => {
                        matches.insert(tile.id, 1);
                    },
                }
                match matches.get_mut(&check_tile.id) {
                    Some(v) => *v += 1,
                    None => {
                        matches.insert(check_tile.id, 1);
                    },
                }
            }
        }
    }

    matches.iter().filter(|(_, count)| **count == 2).map(|(id, _)| *id).collect::<Vec<u16>>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input: Vec<String> = fs::read_to_string(filename)
        .expect(&format!("Could not open file: {}", filename))
        .lines()
        .map(String::from)
        .collect();

    let mut tiles = vec![];
    let mut i = 0;

    while i < input.len() {
        let line = &input[i];
        let mut tile = Tile::new(line);
        i += 1;
        while i < input.len() {
            let image_line = &input[i];
            i += 1;
            
            if image_line.len() == 0 {
                break;
            }

            tile.add_image_line(image_line);
        }
        tile.calculate_borders();
        tiles.push(tile);
    }

    for tile in tiles.iter() {
        // println!("{}", tile);
    }

    let corners = find_corners(&tiles);
    println!("Found {} corners: {:?}", corners.len(), corners);
    println!("Found {} corners with product: {}", corners.len(), corners.iter().map(|id| *id as u64).product::<u64>());
}