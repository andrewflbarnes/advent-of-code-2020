use std::env;
use std::fs;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
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
        if self.borders.len() > 0 {
            self.borders = vec![];
        }
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

    fn adjacent_to(&self, other: &Self) -> (bool, BorderSide, BorderSide, bool) {
        for i in 0..4 {
            let border_side = match i {
                0 => BorderSide::Top,
                1 => BorderSide::Right,
                2 => BorderSide::Bottom,
                3 => BorderSide::Left,
                _ => panic!("Invalid: {}", i),
            };
            for j in 0..other.borders.len() {
                let other_border_side = match j {
                    0 | 4 => BorderSide::Top,
                    1 | 5 => BorderSide::Right,
                    2 | 6 => BorderSide::Bottom,
                    3 | 7 => BorderSide::Left,
                    _ => panic!("Invalid: {}", j),
                };
                let other_border = &other.borders[j];
                if self.borders[i] == other_border.to_owned() {
                    return (true, border_side, other_border_side, j < 4);
                }
            }
        }
        (false, BorderSide::None, BorderSide::None, false)
    }

    fn adjacent_to_side(&self, border_side: BorderSide, other: &Self) -> (bool, BorderSide, bool) {
        let border = match border_side {
            BorderSide::Top => &self.borders[0],
            BorderSide::Right => &self.borders[1],
            BorderSide::Bottom => &self.borders[2],
            BorderSide::Left => &self.borders[3],
            _ => panic!("NONE not allowed"),
        };
        println!("Test {}", border);
        println!("{:?}", other.borders);
        for j in 0..other.borders.len() {
            let other_border_side = match j {
                0 | 4 => BorderSide::Top,
                1 | 5 => BorderSide::Right,
                2 | 6 => BorderSide::Bottom,
                3 | 7 => BorderSide::Left,
                _ => panic!("Invalid: {}", j),
            };
            let other_border = &other.borders[j];
            if border == other_border {
                println!("Border match");
                println!("{} => {} {:?}", self.id, border, border_side);
                println!("{} => {} {:?}", other.id, other_border, other_border_side);
                return (true, other_border_side, j < 4);
            }
        }
        (false, BorderSide::None, false)
    }

    fn transform(&mut self, t: Transform) {
        println!("Transform {:?} on {} from", t, self.id);
        // for l in self.image.iter() {
        //     println!("{}", l);
        // }
        match t {
            Transform::FlipVert => {
                let mut updated = vec![];
                while let Some(l) = self.image.pop() {
                    updated.push(l);
                }
                self.image = updated;
            },
            Transform::FlipHori => {
                let mut updated = vec![];
                for i in 0..self.image.len() {
                    updated.push(self.image[i].chars().rev().collect());
                }
                self.image = updated;
            },
            Transform::Rot90 => {
                let mut updated = vec![];
                let len = self.image.len();
                let width = self.image[0].len();
                for j in 0..width {
                    let mut line = String::new();
                    for i in 1..=len {
                        line.push(self.image[len - i].chars().nth(j).unwrap());
                    }
                    updated.push(line);
                }
                self.image = updated;
            },
            Transform::Rot180 => {
                let mut updated = vec![];
                while let Some(l) = self.image.pop() {
                    updated.push(l.chars().rev().collect());
                }
                self.image = updated;
            },
            Transform::Rot270 => {
                let mut updated = vec![];
                let len = self.image.len();
                let width = self.image[0].len();
                for j in 1..=width {
                    let mut line = String::new();
                    for i in 0..len {
                        line.push(self.image[i].chars().nth(width - j).unwrap());
                    }
                    updated.push(line);
                }
                self.image = updated;
            },
        }
        // println!("to");
        // for l in self.image.iter() {
        //     println!("{}", l);
        // }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BorderSide {
    Top,
    Right,
    Bottom,
    Left,
    None,
}

#[derive(Clone, Copy, Debug)]
enum Transform {
    FlipVert,
    FlipHori,
    Rot90,
    Rot180,
    Rot270,
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
            if tile.adjacent_to(&check_tile).0 {
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

    for (k, v) in matches.iter() {
        println!("tile {} matched {} times", k , v);
    }

    matches.iter().filter(|(_, count)| **count == 2).map(|(id, _)| *id).collect::<Vec<u16>>()
}

fn build_image(size: u8, tiles: &mut Vec<Tile>, start_corner: u16) -> Vec<Tile> {
    let mut tile_path: Vec<Tile> = vec![];
    let mut idx = tiles.iter().position(|t| t.id == start_corner).unwrap();
    let mut tile = tiles.remove(idx);

    // determine image directions
    let mut matched_sides = vec![];
    for check_tile in tiles.iter() {
        if tile == *check_tile {
            continue;
        }

        let (adj, tile_dir, _, _) = tile.adjacent_to(check_tile);
        if adj {
            matched_sides.push(tile_dir);
            if matched_sides.len() == 2 {
                break;
            }
        }
    }

    if matched_sides.contains(&BorderSide::Top) {
        if matched_sides.contains(&BorderSide::Left) {
            tile.transform(Transform::Rot180);
        } else {
            tile.transform(Transform::Rot90);
        }
    } else if matched_sides.contains(&BorderSide::Left) {
        tile.transform(Transform::Rot270);
    } else {
        println!("perfect match");
    }
    tile.calculate_borders();
    tile_path.push(tile);

    while tiles.len() > 0 {
        let tile: &Tile;
        let side: BorderSide;
        if tile_path.len() % size as usize > 0 {
            side = BorderSide::Right;
            tile = &tile_path[tile_path.len() - 1];
        } else {
            side = BorderSide::Bottom;
            tile = &tile_path[tile_path.len() - size as usize];
        }
        println!("Path len {} with most recent {} - testing side {:?} on loop {}", tile_path.len(), tile.id, side, tile_path.len());
        idx = tiles.iter().position(|t| tile.adjacent_to_side(side, t).0).unwrap();
        let mut next = tiles.remove(idx);
        println!("Match prev tile {} to {} at index {}", tile.id, next.id, idx);
        let (adj, to_side, flip) = tile.adjacent_to_side(side, &next);
        match (side, to_side) {
            (BorderSide::Right, BorderSide::Top) => next.transform(Transform::Rot270),
            (BorderSide::Right, BorderSide::Right) => next.transform(Transform::Rot180),
            (BorderSide::Right, BorderSide::Bottom) => next.transform(Transform::Rot90),
            (BorderSide::Bottom, BorderSide::Right) => next.transform(Transform::Rot270),
            (BorderSide::Bottom, BorderSide::Bottom) => next.transform(Transform::Rot180),
            (BorderSide::Bottom, BorderSide::Left) => next.transform(Transform::Rot90),
            _ => {},
        }
        if flip {
            if side == BorderSide::Right {
                next.transform(Transform::FlipVert);
            } else {
                next.transform(Transform::FlipHori);
            }
        }
        next.calculate_borders();
        tile_path.push(next);
    }

    tile_path
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

    let corners = find_corners(&tiles);
    println!("Found {} corners {:?} with product {}", corners.len(), corners, corners.iter().map(|id| *id as u64).product::<u64>());

    let path = build_image(12, &mut tiles, corners[0]);

    for i in 0..12 {
        for j in 0..12 {
            println!("{}", path[(i * 12) + j]);
        }
    }
}