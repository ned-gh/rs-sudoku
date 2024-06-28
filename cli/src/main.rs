mod grid;
mod solver;
mod strategy;
mod util;

use grid::Grid;

fn main() {
    let bd = "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
    let grid = Grid::from_str(bd).unwrap();

    let hsingles = strategy::find_hidden_single(&grid);
    let nsingles = strategy::find_naked_single(&grid);

    println!("Hidden singles: {:?}", hsingles);
    println!("Naked singles: {:?}", nsingles);
}
