use super::StrategyResult;
use crate::grid::{CellCandidate, Grid, UnitType};

use UnitType::{Col, MiniGrid, Row};

pub fn find_hidden_single(grid: &Grid) -> Option<StrategyResult> {
    let mut singles = vec![];

    for val in 1..10 {
        for k in 0..9 {
            for unit_type in &[Row, Col, MiniGrid] {
                let cells = grid.get_unit(unit_type, k).scan(val);
                if cells.len() == 1 {
                    let cell = cells.get_single();

                    singles.push(CellCandidate::from_cell(&cell, val));
                }
            }
        }
    }

    if singles.is_empty() {
        None
    } else {
        Some(StrategyResult::from(
            "Hidden Single",
            singles,
            vec![],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hidden_singles() {
        let bd =
            "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
        let grid = Grid::from_str(bd).unwrap();

        let expected = CellCandidate::from(4, 8, 1);

        let single = find_hidden_single(&grid).unwrap();
        let to_place = single.get_to_place().clone();
        let to_eliminate = single.get_to_eliminate().clone();

        assert!(to_place.contains(&expected));
        assert_eq!(Vec::<CellCandidate>::new(), to_eliminate);
    }
}
