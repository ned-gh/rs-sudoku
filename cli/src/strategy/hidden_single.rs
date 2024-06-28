use std::collections::HashSet;

use crate::grid::{Grid, CellCandidate, UnitType};

use UnitType::{Row, Col, MiniGrid};


pub fn find_hidden_single(grid: &Grid) -> Option<Vec<CellCandidate>> {
    let mut singles = HashSet::new();

    for val in 1..10 {
        for k in 0..9 {
            for unit_type in &[Row, Col, MiniGrid] {
                let cells = grid.get_unit(unit_type, k).scan(val);
                if cells.len() == 1 {
                    let cell = cells.get_single();
                    singles.insert(CellCandidate::from_cell(&cell, val));
                }
            }
        }
    }

    if singles.is_empty() {
        None
    } else {
        Some(singles.into_iter().collect())
    }
}
