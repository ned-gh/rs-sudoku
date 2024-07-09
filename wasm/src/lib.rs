use wasm_bindgen::prelude::*;

use lib::{
    grid::Grid,
    solver::Solver,
};

#[wasm_bindgen]
pub fn get_grid_from_bd_str(bd: &str) -> JsValue {
    let grid_res = Grid::from_str(bd);

    let grid = match grid_res {
        Ok(g) => Some(g),
        Err(_) => None,
    };

    serde_wasm_bindgen::to_value(&grid).unwrap()
}

#[wasm_bindgen]
pub fn solve_step(grid_obj: JsValue) -> JsValue {
    let grid: Grid = serde_wasm_bindgen::from_value(grid_obj).unwrap();

    let mut solver = Solver::from(grid);

    let step = solver.step();
    let new_grid = match &step {
        Some(res) => {
            solver.apply(res);
            Some(solver.get_grid().clone())
        },
        None => None,
    };

    serde_wasm_bindgen::to_value(&(&step, &new_grid)).unwrap()
}
