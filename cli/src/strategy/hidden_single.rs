use std::collections::HashSet;

use crate::grid::{Grid, CellCandidate, UnitType};
use super::StrategyResult;

use UnitType::{Row, Col, MiniGrid};


pub fn find_hidden_singles(grid: &Grid) -> Option<StrategyResult> {
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
        Some(StrategyResult::from(singles.into_iter().collect(), vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hidden_singles() {
        let bd = "000000000904607000076804100309701080008000300050308702007502610000403208000000000";
        let grid = Grid::from_str(bd).unwrap();

        let mut expected = vec![
            CellCandidate::from(4, 8, 1),
            CellCandidate::from(4, 0, 7),
        ];
        expected.sort();

        let result = find_hidden_singles(&grid).unwrap();
        let mut to_place = result.get_to_place().clone();
        let mut to_eliminate = result.get_to_eliminate().clone();

        to_place.sort();
        to_eliminate.sort();

        assert_eq!(expected, to_place);
        assert_eq!(Vec::<CellCandidate>::new(), to_eliminate);
    }
}
