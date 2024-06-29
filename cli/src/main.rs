mod grid;
mod solver;
mod strategy;
mod util;

fn main() {
    let bd1 = "300009000000001020056002790003000048000040107009000000000000000080360200500820400";
    let bd2 = "000700001000090800408600000009052000020001700050000000300000090960000070000000125";

    let grid = grid::Grid::from_str(bd2).unwrap();
    let mut solver = solver::Solver::from(grid);

    println!("{}", solver.get_grid());

    loop {
        let Some(res) = solver.step() else {
            break;
        };

        solver.apply(&res);
        println!("{:?}", res);
        println!("{}", solver.get_grid());
    }

    println!("*****");
    println!("{}", solver.get_grid());
}
