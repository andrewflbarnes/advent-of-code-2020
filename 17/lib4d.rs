use std::fmt;

pub struct State {
    domain: Vec<Vec<Vec<Vec<u8>>>>,
    x_bound: usize,
    y_bound: usize,
    z_bound: usize,
    w_bound: usize,
}

impl State {
    pub fn new(init_state: Vec<String>, bound: usize) -> State {

        let extends = (bound + 1) * 2;
        let x_init = init_state[0].len();
        let y_init = init_state.len();
        let x_bound = x_init + extends;
        let y_bound = y_init + extends;
        let z_bound = 1 + extends;
        let w_bound = 1 + extends;
        let insertion_start = bound + 1;
    
        let mut domain: Vec<Vec<Vec<Vec<u8>>>> = vec![vec![vec![vec![0; w_bound]; z_bound]; y_bound]; x_bound];
    
        // println!("Bounds: x={}, y={}, z={} w={}", x_bound, y_bound, z_bound, w_bound);
    
        for y in 0..init_state.len() {
            let line = &init_state[y];
            let mut chars = line.chars();
            let mut x = 0;
            while let Some(ch) = chars.next() {
                let val = match ch {
                    '#' => 1,
                    _ => 0,
                };
                domain[x + insertion_start][y + insertion_start][insertion_start][insertion_start] = val;
                x += 1;
            }
        }

        State {
            domain,
            x_bound,
            y_bound,
            z_bound,
            w_bound,
        }
    }

    pub fn cycle(&mut self) {
        let mut updated: Vec<Vec<Vec<Vec<u8>>>> = vec![vec![vec![vec![0; self.w_bound]; self.z_bound]; self.y_bound]; self.x_bound];
        for x in 1..(self.x_bound - 1) {
            for y in 1..(self.y_bound - 1) {
                for z in 1..(self.z_bound - 1) {
                    for w in 1..(self.w_bound - 1) {
                        let mut activity = 0;
                        let mut active = 0;
                        for xoff in (x-1)..=(x+1) {
                            for yoff in (y-1)..=(y+1) {
                                for zoff in (z-1)..=(z+1) {
                                    for woff in (w-1)..=(w+1) {
                                        if xoff == x && yoff == y && zoff == z && woff == w {
                                            active = self.domain[xoff][yoff][zoff][woff];
                                        } else {
                                            activity += self.domain[xoff][yoff][zoff][woff];
                                        }
                                    }
                                }
                            }
                        }

                        // println!("cell={}-{}-{} active={} activity={}", x, y, z, active, activity);
                        if active == 1 && (activity < 2 || activity > 3) {
                            updated[x][y][z][w] = 0;
                        } else if active == 0 && activity == 3 {
                            updated[x][y][z][w] = 1;
                        } else {
                            updated[x][y][z][w] = active;
                        }
                    }
                }
            }
        }

        self.domain = updated;
    }

    pub fn count_active(&self) -> u64 {
        let mut count = 0;

        for x in 1..(self.x_bound - 1) {
            for y in 1..(self.y_bound - 1) {
                for z in 1..(self.z_bound - 1) {
                    for w in 1..(self.w_bound - 1) {
                        count += self.domain[x][y][z][w] as u64;
                    }
                }
            }
        }

        count
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        // for z in 7..8 {
        for w in 0..self.w_bound {
            for z in 0..self.z_bound {
                println!("Layer z={} w={}", z, w);
                for y in 0..self.y_bound {
                    for x in 0..self.x_bound {
                        let ch = match self.domain[x][y][z][w] {
                            0 => '.',
                            _ => '#',
                        };
                        write!(f, "{}", ch)?;
                    }
                    write!(f, "\n")?;
                }
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}