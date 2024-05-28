use cells::Cells;
use std::{thread, time};

mod cells;

const W: usize = 1000;
const H: usize = 1000;
const CELLS_COUNT: usize = W * H;

fn main() {
    let lapse = time::Duration::from_millis(100);
    let mut cells = Cells::new();
    let mut prev_cells: [Cells; 2] = [Cells::new(); 2];

    cells.randomise()
;
    loop {
        for prev_cell in prev_cells {
            if prev_cell == cells {
                cells.randomise()
            }
        }
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", cells);
        thread::sleep(lapse);
        prev_cells [0] = prev_cells[1];
        prev_cells[1] = cells;
        cells.advance();
    }
}
