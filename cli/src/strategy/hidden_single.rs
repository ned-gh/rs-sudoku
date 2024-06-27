use std::collections::HashSet;

use crate::grid::{Grid, cell::CellCandidate};


pub fn find_hidden_single(grid: &Grid) -> Option<Vec<CellCandidate>> {
    let mut singles = HashSet::new();

    for val in 1..10 {
        for k in 0..9 {
            let mut cells = grid.scan_row(k, val);
            if cells.len() == 1 {
                singles.insert(cells[0].clone());
            }

            cells = grid.scan_col(k, val);
            if cells.len() == 1 {
                singles.insert(cells[0].clone());
            }

            cells = grid.scan_box_n(k, val);
            if cells.len() == 1 {
                singles.insert(cells[0].clone());
            }
        }
    }

    if singles.is_empty() {
        None
    } else {
        Some(singles.into_iter().collect())
    }
}
