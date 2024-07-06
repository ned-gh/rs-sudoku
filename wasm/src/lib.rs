use wasm_bindgen::prelude::*;

use lib::{
    grid::Grid,
    solver::Solver,
};

#[wasm_bindgen]
pub fn get_grid_from_bd_str(bd: &str) -> JsValue {
    let grid = Grid::from_str(bd).unwrap();

    serde_wasm_bindgen::to_value(&grid).unwrap()
}

#[wasm_bindgen]
pub fn solve_step(grid_obj: JsValue) -> JsValue {
    let grid: Grid = serde_wasm_bindgen::from_value(grid_obj).unwrap();

    let solver = Solver::from(grid);

    serde_wasm_bindgen::to_value(&solver.step()).unwrap()
}
