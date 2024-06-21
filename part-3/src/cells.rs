use rand::Rng;

use crate::{ CELLS_COUNT, H, W };

#[derive(Clone, Copy)]
pub struct Cell {
    pub is_alive: bool,
    neighbor_indices: [Option<usize>; 8],
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_alive { write!(f, "● ") } else { write!(f, "  ") }
    }
}

fn get_neighbor_indices(index: usize) -> [Option<usize>; 8] {
    let mut neighbor_indices: [Option<usize>; 8] = [None; 8];
    let w_mod = index % W;
    let is_left_edge = w_mod == 0;
    let is_right_edge = w_mod == W - 1;
    let is_top_edge = index < W;
    let is_bottom_edge = index >= H * (W - 1);

    if !is_left_edge {
        neighbor_indices[3] = Some(index - 1);
    }
    if !is_right_edge {
        neighbor_indices[4] = Some(index + 1);
    }
    if !is_top_edge {
        if !is_left_edge {
            neighbor_indices[0] = Some(index - W - 1);
        }
        neighbor_indices[1] = Some(index - W);
        if !is_right_edge {
            neighbor_indices[2] = Some(index - W + 1);
        }
    }
    if !is_bottom_edge {
        if !is_left_edge {
            neighbor_indices[5] = Some(index + W - 1);
        }
        neighbor_indices[6] = Some(index + W);
        if !is_right_edge {
            neighbor_indices[7] = Some(index + W + 1);
        }
    }
    neighbor_indices
}

#[derive(Clone, Copy)]
pub struct Cells {
    pub values: [Cell; CELLS_COUNT],
    living_neighbors: [u8; CELLS_COUNT],
}

impl Cells {
    fn populate_neighbor_indices(&mut self) {
        for (ii, cell) in self.values.iter_mut().enumerate() {
            cell.neighbor_indices = get_neighbor_indices(ii);
        }
    }

    fn update_living_neighbors(&mut self) {
        for (ii, cell) in self.values.iter().enumerate() {
            self.living_neighbors[ii] = 0;
            for neighbor_index in cell.neighbor_indices {
                neighbor_index.inspect(|ni| {
                    if self.values[*ni].is_alive {
                        self.living_neighbors[ii] += 1;
                    }
                });
            }
        }
    }

    pub fn advance(&mut self) {
        self.update_living_neighbors();
        for (ii, cell) in self.values.iter_mut().enumerate() {
            if cell.is_alive && (self.living_neighbors[ii] < 2 || self.living_neighbors[ii] >= 4) {
                cell.is_alive = false;
            } else if !cell.is_alive && self.living_neighbors[ii] == 3 {
                cell.is_alive = true;
            }
        }
    }

    pub fn randomise(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.values.iter_mut() {
            cell.is_alive = (rng.gen::<f64>().round() as u8) == 0;
        }
    }

    pub fn new() -> Cells {
        let mut cells = Cells {
            values: [
                Cell {
                    is_alive: false,
                    neighbor_indices: [None; 8],
                };
                CELLS_COUNT
            ],
            living_neighbors: [0; CELLS_COUNT],
        };

        cells.populate_neighbor_indices();

        cells
    }
}

impl std::fmt::Display for Cells {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string: String = "".to_string();
        for (ii, cell) in self.values.iter().enumerate() {
            if ii % W == 0 {
                string.push_str("\n");
            }
            string.push_str(&cell.to_string());
        }
        write!(f, "{string}")
    }
}

impl std::cmp::PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.is_alive == other.is_alive;
    }
}

impl std::cmp::PartialEq for Cells {
    fn eq(&self, other: &Self) -> bool {
        for (ii, cell) in self.values.iter().enumerate() {
            if *cell != other.values[ii] {
                return false;
            }
        }
        true
    }
}
