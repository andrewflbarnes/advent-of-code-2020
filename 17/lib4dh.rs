use std::fmt;
use std::collections::HashSet;

pub struct State {
    domain: HashSet<(usize, usize, usize, usize)>,
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

        let mut domain: HashSet<(usize, usize, usize, usize)> = HashSet::new();

        // println!("Bounds: x={}, y={}, z={} w={}", x_bound, y_bound, z_bound, w_bound);

        for y in 0..init_state.len() {
            let line = &init_state[y];
            let mut chars = line.chars();
            let mut x = 0;
            while let Some(ch) = chars.next() {
                if ch == '#' {
                    domain.insert((x + insertion_start, y + insertion_start, insertion_start, insertion_start));
                }
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
        let mut updated: HashSet<(usize, usize, usize, usize)> = HashSet::new();
        let mut checked: HashSet<(usize, usize, usize, usize)> = self.domain.clone();
        for cell in self.domain.iter() {
            // check the cell itself
            let (x, y, z, w) = cell;
            self.cell_update(true, &mut checked, &mut updated, *x, *y, *z, *w);
            // check surrounding cells
        }

        self.domain = updated;
    }

    fn cell_update(&self, check_neighbours: bool, checked_cells: &mut HashSet<(usize, usize, usize, usize)>, active_cells: &mut HashSet<(usize, usize, usize, usize)>, x: usize, y: usize, z: usize, w: usize) {
        let mut active = false;
        let mut activity = 0;
        for xoff in (x-1)..=(x+1) {
            for yoff in (y-1)..=(y+1) {
                for zoff in (z-1)..=(z+1) {
                    for woff in (w-1)..=(w+1) {
                        let cell = (xoff, yoff, zoff, woff);
                        let cell_active = self.domain.contains(&cell);
                        if (x, y, z, w) == cell {
                            active = cell_active;
                        } else if cell_active {
                            activity += 1;
                        }

                        if check_neighbours && !checked_cells.contains(&cell) {
                            self.cell_update(false, checked_cells, active_cells, xoff, yoff, zoff, woff);
                            checked_cells.insert(cell);
                        }
                    }
                }
            }
        }

        if (active && activity >= 2 && activity <= 3) || (!active && activity == 3) {
            active_cells.insert((x, y, z, w));
        }
    }

    pub fn count_active(&self) -> u64 {
        self.domain.len() as u64
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for w in 0..self.w_bound {
            for z in 0..self.z_bound {
                println!("Layer z={} w={}", z, w);
                for y in 0..self.y_bound {
                    for x in 0..self.x_bound {
                        if self.domain.contains(&(x, y, z, w)) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    write!(f, "\n")?;
                }
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}