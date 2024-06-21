use cells::Cells;
use window::run;

mod cells;
mod window;

const W: usize = 50;
const H: usize = 50;
const CELLS_COUNT: usize = W * H;

fn main() {
    let mut cells = Cells::new();

    cells.randomise();

    run(cells);
}
